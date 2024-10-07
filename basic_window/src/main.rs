#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raylib::*;

fn main() {
    unsafe {
        const SCREEN_WIDTH: i32 = 800;
        const SCREEN_HEIGHT: i32 = 450;

        InitWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            c"raylib [core] example - basic window".as_ptr(),
        );
        SetTargetFPS(60);

        while WindowShouldClose() == false {
            BeginDrawing();
            ClearBackground(RAYWHITE);
            DrawText(
                c"Congrats! You created your first window!".as_ptr(),
                190,
                200,
                20,
                LIGHTGRAY,
            );
            EndDrawing();
        }

        CloseWindow();
    }
}
