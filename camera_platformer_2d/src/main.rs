#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raylib::*;

const G: i32 = 400;
const PLAYER_JUMP_SPEED: f32 = 350.0;
const PLAYER_HORIZONTAL_SPEED: f32 = 200.0;

struct Player {
    position: Vector2,
    speed: f32,
    can_jump: bool,
}

struct EnvItem {
    rect: Rectangle,
    blocking: i32,
    color: Color,
}

enum CameraOption {
    FollowCenter,
    FollowCenterClamp,
    FollowCenterSmooth,
    FollowCenterHorizontal,
    PlayerPush,
}

fn main() {
    unsafe { render() };
}

unsafe fn render() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    InitWindow(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        c"raylib [core] example - 2d camera".as_ptr(),
    );

    let mut player = Player {
        position: Vector2 { x: 400.0, y: 280.0 },
        speed: 0.0,
        can_jump: false,
    };

    let env_items = [
        EnvItem {
            rect: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1000.0,
                height: 400.0,
            },
            blocking: 0,
            color: LIGHTGRAY,
        },
        EnvItem {
            rect: Rectangle {
                x: 0.0,
                y: 400.0,
                width: 1000.0,
                height: 200.0,
            },
            blocking: 1,
            color: GRAY,
        },
        EnvItem {
            rect: Rectangle {
                x: 300.0,
                y: 200.0,
                width: 400.0,
                height: 10.0,
            },
            blocking: 1,
            color: GRAY,
        },
        EnvItem {
            rect: Rectangle {
                x: 250.0,
                y: 300.0,
                width: 100.0,
                height: 10.0,
            },
            blocking: 1,
            color: GRAY,
        },
        EnvItem {
            rect: Rectangle {
                x: 650.0,
                y: 300.0,
                width: 100.0,
                height: 10.0,
            },
            blocking: 1,
            color: GRAY,
        },
    ];

    let mut camera = Camera2D {
        offset: Vector2 {
            x: SCREEN_WIDTH as f32 / 2.0,
            y: SCREEN_HEIGHT as f32 / 2.0,
        },
        target: player.position,
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut camera_option = CameraOption::FollowCenter;
    let camera_descriptions = [
        c"Follow player center".as_ptr(),
        c"Follow player center, but clamp to map edges".as_ptr(),
        c"Follow player center; smoothed".as_ptr(),
        c"Follow player center horizontally; update player center vertically after landing"
            .as_ptr(),
        c"Player push camera on getting too close to screen edge".as_ptr(),
    ];

    SetTargetFPS(60);
    while WindowShouldClose() == false {
        let delta_time = GetFrameTime();

        update_player(&mut player, &env_items, delta_time);

        camera.zoom += GetMouseWheelMove() * 0.05;
        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        }
        if camera.zoom < 0.25 {
            camera.zoom = 0.25;
        }

        if IsKeyPressed(KeyboardKey_KEY_R) {
            camera.zoom = 1.0;
            player.position = Vector2 { x: 400.0, y: 280.0 };
        }

        if IsKeyPressed(KeyboardKey_KEY_C) {
            camera_option = match camera_option {
                CameraOption::FollowCenter => CameraOption::FollowCenterClamp,
                CameraOption::FollowCenterClamp => CameraOption::FollowCenterSmooth,
                CameraOption::FollowCenterSmooth => CameraOption::FollowCenterHorizontal,
                CameraOption::FollowCenterHorizontal => CameraOption::PlayerPush,
                CameraOption::PlayerPush => CameraOption::FollowCenter,
            };
        }

        match camera_option {
            CameraOption::FollowCenter => update_camera_center(
                &mut camera,
                &player,
                &env_items,
                delta_time,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            ),
            CameraOption::FollowCenterClamp => update_camera_center_inside_map(
                &mut camera,
                &player,
                &env_items,
                delta_time,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            ),
            CameraOption::FollowCenterSmooth => update_camera_center_smooth_follow(
                &mut camera,
                &player,
                &env_items,
                delta_time,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            ),
            CameraOption::FollowCenterHorizontal => update_camera_even_out_on_landing(
                &mut camera,
                &player,
                &env_items,
                delta_time,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            ),
            CameraOption::PlayerPush => update_camera_player_bounds_push(
                &mut camera,
                &player,
                &env_items,
                delta_time,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            ),
        }
        BeginDrawing();

        ClearBackground(LIGHTGRAY);
        BeginMode2D(camera);

        for env_item in &env_items {
            DrawRectangleRec(env_item.rect, env_item.color);
        }

        let player_rect = Rectangle {
            x: player.position.x - 20.0,
            y: player.position.y - 40.0,
            width: 40.0,
            height: 40.0,
        };
        DrawRectangleRec(player_rect, RED);
        DrawCircleV(player.position, 5.0, GOLD);
        EndMode2D();

        DrawText(c"Controls:".as_ptr(), 20, 20, 10, BLACK);
        DrawText(c"- Right/Left to move".as_ptr(), 40, 40, 10, DARKGRAY);
        DrawText(c"- Space to jump".as_ptr(), 40, 60, 10, DARKGRAY);
        DrawText(
            c"- Mouse Wheel to Zoom in-out, R to reset zoom".as_ptr(),
            40,
            80,
            10,
            DARKGRAY,
        );
        DrawText(c"- C to change camera mode".as_ptr(), 40, 100, 10, DARKGRAY);
        DrawText(c"Current camera mode:".as_ptr(), 20, 120, 10, BLACK);
        let desc = match camera_option {
            CameraOption::FollowCenter => camera_descriptions[0],
            CameraOption::FollowCenterClamp => camera_descriptions[1],
            CameraOption::FollowCenterSmooth => camera_descriptions[2],
            CameraOption::FollowCenterHorizontal => camera_descriptions[3],
            CameraOption::PlayerPush => camera_descriptions[4],
        };
        DrawText(desc, 40, 140, 10, DARKGRAY);

        EndDrawing();
    }

    CloseWindow();
}

unsafe fn update_player(player: &mut Player, env_items: &[EnvItem], delta_time: f32) {
    if IsKeyDown(KeyboardKey_KEY_LEFT) {
        player.position.x -= PLAYER_HORIZONTAL_SPEED * delta_time;
    }
    if IsKeyDown(KeyboardKey_KEY_RIGHT) {
        player.position.x += PLAYER_HORIZONTAL_SPEED * delta_time;
    }
    if IsKeyDown(KeyboardKey_KEY_SPACE) && player.can_jump {
        player.speed = -PLAYER_JUMP_SPEED;
        player.can_jump = false;
    }

    let mut hit_obsticle = false;
    for env_item in env_items {
        if env_item.blocking == 1
            && env_item.rect.x <= player.position.x
            && env_item.rect.x + env_item.rect.width >= player.position.x
            && env_item.rect.y >= player.position.y
            && env_item.rect.y <= player.position.y + player.speed * delta_time
        {
            hit_obsticle = true;
            player.speed = 0.0;
            player.position.y = env_item.rect.y;
            break;
        }
    }

    if hit_obsticle == false {
        player.position.y += player.speed * delta_time;
        player.speed += G as f32 * delta_time;
        player.can_jump = false;
    } else {
        player.can_jump = true;
    }
}

unsafe fn update_camera_center(
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    _delta_time: f32,
    width: i32,
    height: i32,
) {
    camera.offset = Vector2 {
        x: width as f32 / 2.0,
        y: height as f32 / 2.0,
    };
    camera.target = player.position;
}

unsafe fn update_camera_center_inside_map(
    camera: &mut Camera2D,
    player: &Player,
    env_items: &[EnvItem],
    _delta_time: f32,
    width: i32,
    height: i32,
) {
    camera.offset = Vector2 {
        x: width as f32 / 2.0,
        y: height as f32 / 2.0,
    };
    camera.target = player.position;

    let mut min_x = 1000.0f32;
    let mut min_y = 1000.0f32;
    let mut max_x = -1000.0f32;
    let mut max_y = -1000.0f32;

    for env_item in env_items.into_iter().skip(1) {
        min_x = min_x.min(env_item.rect.x);
        max_x = max_x.max(env_item.rect.x + env_item.rect.width);
        min_y = min_y.min(env_item.rect.y);
        max_y = max_y.max(env_item.rect.y + env_item.rect.height);
    }

    let max = GetWorldToScreen2D(Vector2 { x: max_x, y: max_y }, *camera);
    let min = GetWorldToScreen2D(Vector2 { x: min_x, y: min_y }, *camera);

    if max.x < width as f32 {
        camera.offset.x = width as f32 - (max.x - (width as f32 / 2.0));
    }
    if max.y < height as f32 {
        camera.offset.y = height as f32 - (max.y - (height as f32 / 2.0));
    }
    if min.x > 0.0 {
        camera.offset.x = (width as f32 / 2.0) - min.x;
    }
    if min.y > 0.0 {
        camera.offset.y = (height as f32 / 2.0) - min.y;
    }
}

unsafe fn update_camera_center_smooth_follow(
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    delta_time: f32,
    width: i32,
    height: i32,
) {
    const MIN_SPEED: f32 = 30.0;
    const MIN_EFFECT_LENGTH: f32 = 10.0;
    const FRACTION_SPEED: f32 = 0.8;

    camera.offset = Vector2 {
        x: width as f32 / 2.0,
        y: height as f32 / 2.0,
    };

    let diff = Vector2Subtract(player.position, camera.target);
    let length = Vector2Length(diff);

    if length > MIN_EFFECT_LENGTH {
        let speed = MIN_SPEED.max(FRACTION_SPEED * length);
        camera.target = Vector2Add(
            camera.target,
            Vector2Scale(diff, speed * delta_time / length),
        );
    }
}

unsafe fn update_camera_even_out_on_landing(
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    delta_time: f32,
    width: i32,
    height: i32,
) {
    const EVEN_OUT_SPEED: f32 = 700.0;
    static mut EVENING_OUT: bool = false;
    static mut EVENING_OUT_TARGET: f32 = 0.0;

    camera.offset = Vector2 {
        x: width as f32 / 2.0,
        y: height as f32 / 2.0,
    };
    camera.target.x = player.position.x;

    if EVENING_OUT {
        if EVENING_OUT_TARGET > camera.target.y {
            camera.target.y += EVEN_OUT_SPEED * delta_time;
            if camera.target.y > EVENING_OUT_TARGET {
                camera.target.y = EVENING_OUT_TARGET;
                EVENING_OUT = false;
            }
        } else {
            camera.target.y -= EVEN_OUT_SPEED * delta_time;
            if camera.target.y < EVENING_OUT_TARGET {
                camera.target.y = EVENING_OUT_TARGET;
                EVENING_OUT = false;
            }
        }
    } else {
        if player.can_jump && player.speed == 0.0 && player.position.y != camera.target.y {
            EVENING_OUT = true;
            EVENING_OUT_TARGET = player.position.y;
        }
    }
}

unsafe fn update_camera_player_bounds_push(
    camera: &mut Camera2D,
    player: &Player,
    _env_items: &[EnvItem],
    _delta_time: f32,
    width: i32,
    height: i32,
) {
    static mut BBOX: Vector2 = Vector2 { x: 0.2, y: 0.2 };

    let bbox_world_min = GetScreenToWorld2D(
        Vector2 {
            x: (1.0 - BBOX.x) * 0.5 * (width as f32),
            y: (1.0 - BBOX.y) * 0.5 * (height as f32),
        },
        *camera,
    );

    let bbox_world_max = GetScreenToWorld2D(
        Vector2 {
            x: (1.0 + BBOX.x) * 0.5 * (width as f32),
            y: (1.0 + BBOX.y) * 0.5 * (height as f32),
        },
        *camera,
    );

    camera.offset = Vector2 {
        x: (1.0 - BBOX.x) * 0.5 * (width as f32),
        y: (1.0 - BBOX.y) * 0.5 * (height as f32),
    };

    if player.position.x < bbox_world_min.x {
        camera.target.x = player.position.x;
    }
    if player.position.y < bbox_world_min.y {
        camera.target.y = player.position.y;
    }
    if player.position.x > bbox_world_max.x {
        camera.target.x = bbox_world_min.x + (player.position.x - bbox_world_max.x);
    }
    if player.position.y > bbox_world_max.y {
        camera.target.y = bbox_world_min.y + (player.position.y - bbox_world_max.y);
    }
}
