#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raylib::*;

fn main() {
    unsafe { render() };
}

unsafe fn render() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    InitWindow(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        c"raylib [textures] example - fog of war".as_ptr(),
    );

    while WindowShouldClose() == false {
        BeginDrawing();
        EndDrawing();
    }

    CloseWindow();
}
