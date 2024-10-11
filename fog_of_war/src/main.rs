#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

use std::{
	ffi::CString,
	thread,
	time::{Duration, Instant},
};

use bevy_ecs::{
	prelude::*,
	schedule::{ExecutorKind, ScheduleLabel},
};
use raylib::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const MAX_FPS: i32 = 120;
const TIMESTEP: Duration = Duration::from_micros(15625);

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
struct Update;

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
struct FixedUpdate;

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
struct Render;

const MAP_TILE_SIZE: i32 = 32;
const PLAYER_SIZE: i32 = 16;
const PLAYER_TILE_VISIBILITY: i32 = 2;

#[derive(Component)]
struct Map {
	tiles_x: i32,
	tiles_y: i32,
	tile_ids: Box<[i8]>,
	tile_fog: Box<[i8]>,
	fog_of_war: RenderTexture2D,
}

impl Drop for Map {
	fn drop(&mut self) {
		unsafe { UnloadRenderTexture(self.fog_of_war) };
	}
}

impl Map {
	fn new() -> Self {
		unsafe {
			let mut map = Self {
				tiles_x: 25,
				tiles_y: 15,
				tile_ids: Box::new([0i8; (25 * 15)]),
				tile_fog: Box::new([0i8; (25 * 15)]),
				fog_of_war: LoadRenderTexture(25, 15),
			};
			for i in 0..(map.tiles_x * map.tiles_y) as usize {
				map.tile_ids[i] = (GetRandomValue(0, 1) & 0x0000FF) as i8;
			}
			SetTextureFilter(map.fog_of_war.texture, TextureFilter_TEXTURE_FILTER_BILINEAR);
			map
		}
	}

	fn render(&self) {
		unsafe {
			for y in 0..self.tiles_y {
				for x in 0..self.tiles_x {
					let tile_color = if self.tile_ids[(y * self.tiles_x + x) as usize] == 1 {
						BLUE
					} else {
						Fade(BLUE, 0.9)
					};
					DrawRectangle(x * MAP_TILE_SIZE, y * MAP_TILE_SIZE, MAP_TILE_SIZE, MAP_TILE_SIZE, tile_color);
					DrawRectangleLines(x * MAP_TILE_SIZE, y * MAP_TILE_SIZE, MAP_TILE_SIZE, MAP_TILE_SIZE, Fade(DARKBLUE, 0.5));
				}
			}

			// draw fog of war (scaled to full map, bilinear filtering)
			DrawTexturePro(
				self.fog_of_war.texture,
				Rectangle {
					x: 0.0,
					y: 0.0,
					width: self.fog_of_war.texture.width as f32,
					height: -self.fog_of_war.texture.height as f32,
				},
				Rectangle {
					x: 0.0,
					y: 0.0,
					width: (self.tiles_x * MAP_TILE_SIZE) as f32,
					height: (self.tiles_y * MAP_TILE_SIZE) as f32,
				},
				Vector2 { x: 0.0, y: 0.0 },
				0.0,
				WHITE,
			);
		}
	}

	fn prepare_fow_texture(&self) {
		unsafe {
			BeginTextureMode(self.fog_of_war);
			ClearBackground(BLANK);
			for y in 0..self.tiles_y {
				for x in 0..self.tiles_x {
					if self.tile_fog[(y * self.tiles_x + x) as usize] == 0 {
						DrawRectangle(x, y, 1, 1, BLACK);
					} else if self.tile_fog[(y * self.tiles_x + x) as usize] == 2 {
						DrawRectangle(x, y, 1, 1, Fade(BLACK, 0.8));
					}
				}
			}
			EndTextureMode();
		}
	}
}

#[derive(Component)]
struct Renderable;

#[derive(Component)]
struct Position {
	x: f32,
	y: f32,
}

impl Position {
	fn to_vector2(&self) -> Vector2 {
		Vector2 { x: self.x, y: self.y }
	}

	fn get_tile_position(&self) -> (i32, i32) {
		(
			(self.x as i32 + (MAP_TILE_SIZE / 2)) / MAP_TILE_SIZE,
			(self.y as i32 + (MAP_TILE_SIZE / 2)) / MAP_TILE_SIZE,
		)
	}
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct BundlePlayer {
	player: Player,
	position: Position,
}

impl BundlePlayer {
	fn new() -> Self {
		Self {
			player: Player,
			position: Position { x: 180.0, y: 130.0 },
		}
	}
}

fn render_map(mut map_query: Query<&mut Map>) {
	let map = map_query.single();
	map.prepare_fow_texture();
	map.render();
}

fn render_player(pos_query: Query<&Position, With<Player>>) {
	unsafe {
		let pos = pos_query.single();
		DrawRectangleV(
			pos.to_vector2(),
			Vector2 {
				x: PLAYER_SIZE as f32,
				y: PLAYER_SIZE as f32,
			},
			RED,
		);
	}
}

fn render_overlay(pos_query: Query<&Position, With<Player>>) {
	let (player_tile_x, player_tile_y) = pos_query.single().get_tile_position();
	unsafe {
		DrawFPS(10, 40);
		DrawText(
			TextFormat(c"Current tile: [%i,%i]".as_ptr(), player_tile_x, player_tile_y),
			10,
			10,
			20,
			RAYWHITE,
		);
		DrawText(c"ARROW KEYS to move".as_ptr(), 10, SCREEN_HEIGHT - 25, 20, RAYWHITE);
	}
}

fn handle_input(mut pos_query: Query<&mut Position, With<Player>>, map: Query<&Map>) {
	unsafe {
		let mut pos = pos_query.single_mut();
		let map = map.single();
		// player movement
		if IsKeyDown(KeyboardKey_KEY_RIGHT) {
			pos.x += 5.0;
		}
		if IsKeyDown(KeyboardKey_KEY_LEFT) {
			pos.x -= 5.0;
		}
		if IsKeyDown(KeyboardKey_KEY_DOWN) {
			pos.y += 5.0;
		}
		if IsKeyDown(KeyboardKey_KEY_UP) {
			pos.y -= 5.0;
		}

		// collisions
		// x axis
		if pos.x < 0.0 {
			pos.x = 0.0;
		} else if pos.x + PLAYER_SIZE as f32 > (map.tiles_x * MAP_TILE_SIZE) as f32 {
			pos.x = ((map.tiles_x * MAP_TILE_SIZE) - PLAYER_SIZE) as f32;
		}
		// y axis
		if pos.y < 0.0 {
			pos.y = 0.0;
		} else if pos.y + PLAYER_SIZE as f32 > (map.tiles_y * MAP_TILE_SIZE) as f32 {
			pos.y = ((map.tiles_y * MAP_TILE_SIZE) - PLAYER_SIZE) as f32;
		}
	}
}

fn handle_fog(mut map_query: Query<&mut Map>, pos_query: Query<&Position, With<Player>>) {
	let mut map = map_query.single_mut();
	let player = pos_query.single();
	// previous visited tiles are set to partial fog
	for i in 0..(map.tiles_x * map.tiles_y) as usize {
		if map.tile_fog[i] == 1 {
			map.tile_fog[i] = 2;
		}
	}

	let tiles_x = map.tiles_x;
	let tiles_y = map.tiles_y;
	let (player_tile_x, player_tile_y) = player.get_tile_position();
	// check visibility and update fog
	for y in (player_tile_y - PLAYER_TILE_VISIBILITY)..(player_tile_y + PLAYER_TILE_VISIBILITY) {
		for x in (player_tile_x - PLAYER_TILE_VISIBILITY)..(player_tile_x + PLAYER_TILE_VISIBILITY) {
			if x >= 0 && x < tiles_x && y >= 0 && y < tiles_y {
				map.tile_fog[(y * tiles_x + x) as usize] = 1;
			}
		}
	}
}

fn main() {
	unsafe {
		InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, c"raylib [textures] example - fog of war".as_ptr());

		SetTargetFPS(MAX_FPS);

		let map = Map::new();
		let player = BundlePlayer::new();

		let mut world = World::new();
		world.spawn(player);
		world.spawn(map);

		let mut update = Schedule::new(Update);
		update.set_executor_kind(ExecutorKind::MultiThreaded);
		update.add_systems(handle_input);
		update.add_systems(handle_fog);
		world.add_schedule(update);

		let mut fixed_update = Schedule::new(FixedUpdate);
		fixed_update.set_executor_kind(ExecutorKind::MultiThreaded);
		world.add_schedule(fixed_update);

		let mut render = Schedule::new(Render);
		render.set_executor_kind(ExecutorKind::MultiThreaded);
		render.add_systems(render_map);
		render.add_systems(render_overlay.after(render_map));
		render.add_systems(render_player.after(render_map));
		world.add_schedule(render);

		let mut time_acc = Duration::ZERO;
		let mut frame_time = Duration::ZERO;
		while WindowShouldClose() == false {
			time_acc = time_acc.saturating_add(frame_time);
			while let Some(time) = time_acc.checked_sub(TIMESTEP) {
				world.run_schedule(FixedUpdate);
				time_acc = time;
			}

			world.run_schedule(Update);

			let render_time = Instant::now();
			BeginDrawing();
			ClearBackground(RAYWHITE);

			world.run_schedule(Render);

			EndDrawing();
			frame_time = render_time.elapsed();
		}

		drop(world);
		CloseWindow();
	}
}
