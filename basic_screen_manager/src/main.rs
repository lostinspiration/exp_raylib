#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raylib::*;

enum GameScreen {
    Logo,
    Title,
    GamePlay,
    Ending,
}

fn main() {
    unsafe {
        const SCREEN_WIDTH: i32 = 800;
        const SCREEN_HEIGHT: i32 = 450;

        InitWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            c"raylib [core] example - basic screen manager".as_ptr(),
        );

        let mut current_screen = GameScreen::Logo;
        let mut frames_counter = 0;

        SetTargetFPS(60);

        while WindowShouldClose() == false {
            match current_screen {
                GameScreen::Logo => {
                    frames_counter += 1;

                    // ~2 seconds @60 fps
                    if frames_counter > 120 {
                        current_screen = GameScreen::Title;
                    }
                }
                GameScreen::Title => {
                    if IsKeyPressed(KeyboardKey_KEY_ENTER)
                        || IsGestureDetected(Gesture_GESTURE_TAP as u32)
                    {
                        current_screen = GameScreen::GamePlay;
                    }
                }
                GameScreen::GamePlay => {
                    if IsKeyPressed(KeyboardKey_KEY_ENTER)
                        || IsGestureDetected(Gesture_GESTURE_TAP as u32)
                    {
                        current_screen = GameScreen::Ending;
                    }
                }
                GameScreen::Ending => {
                    if IsKeyPressed(KeyboardKey_KEY_ENTER)
                        || IsGestureDetected(Gesture_GESTURE_TAP as u32)
                    {
                        current_screen = GameScreen::Title;
                    }
                }
            }

            BeginDrawing();
            match current_screen {
                GameScreen::Logo => {
                    DrawText(c"LOGO SCREEN".as_ptr(), 20, 20, 40, LIGHTGRAY);
                    DrawText(c"WAIT for 2 SECONDS".as_ptr(), 290, 220, 10, GRAY);
                }
                GameScreen::Title => {
                    DrawRectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, GREEN);
                    DrawText(c"TITLE SCREEN".as_ptr(), 20, 20, 40, DARKGREEN);
                    DrawText(
                        c"PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN".as_ptr(),
                        120,
                        220,
                        20,
                        DARKGREEN,
                    );
                }
                GameScreen::GamePlay => {
                    DrawRectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, PURPLE);
                    DrawText(c"GAMEPLAY SCREEN".as_ptr(), 20, 20, 40, MAROON);
                    DrawText(
                        c"PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN".as_ptr(),
                        120,
                        220,
                        20,
                        MAROON,
                    );
                }
                GameScreen::Ending => {
                    DrawRectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, BLUE);
                    DrawText(c"ENDING SCREEN".as_ptr(), 20, 20, 40, DARKBLUE);
                    DrawText(
                        c"PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN".as_ptr(),
                        120,
                        220,
                        20,
                        DARKBLUE,
                    );
                }
            }

            EndDrawing();
        }

        CloseWindow();
    }
}
