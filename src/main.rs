#![feature(decl_macro)]

use std::ffi::{c_int, CString};

use raylib::{
    core::math::Vector2,
    ffi::{
        BeginDrawing, ClearBackground, CloseWindow, DrawRectangleLinesEx, DrawText, EndDrawing,
        InitWindow, IsKeyPressed, KeyboardKey, Rectangle, SetTargetFPS, WindowShouldClose,
    },
};

use crate::{
    colors::{DARK_GREEN, GREEN},
    game::Game,
    utils::event_triggered,
};

mod colors;
mod food;
mod game;
mod snake;
mod utils;

static mut ALLOW_MOVE: bool = false;
static mut LAST_UPDATE_TIME: f64 = 0.0;

const CELL_SIZE: f32 = 30.0;
const CELL_COUNT: f32 = 25.0;
const OFFSET: f32 = 75.0;

fn main() {
    println!("Starting the game...");
    let title_str = CString::new("Retro Snake").unwrap();
    unsafe {
        InitWindow(
            (2.0 * OFFSET + CELL_SIZE * CELL_COUNT) as i32,
            (2.0 * OFFSET + CELL_SIZE * CELL_COUNT) as i32,
            title_str.as_ptr(),
        );
        SetTargetFPS(60);
    }

    let mut game = Game::new();

    while !unsafe { WindowShouldClose() } {
        unsafe {
            BeginDrawing();
        }

        if event_triggered(0.2) {
            unsafe {
                ALLOW_MOVE = true;
                game.update();
            }
        }

        if unsafe { IsKeyPressed(KeyboardKey::KEY_UP as c_int) }
            && game.snake.direction.y != 1.0
            && unsafe { ALLOW_MOVE }
        {
            game.snake.direction = Vector2 { x: 0.0, y: -1.0 };
            game.running = true;
            unsafe {
                ALLOW_MOVE = false;
            }
        }

        if unsafe { IsKeyPressed(KeyboardKey::KEY_DOWN as c_int) }
            && game.snake.direction.y != -1.0
            && unsafe { ALLOW_MOVE }
        {
            game.snake.direction = Vector2 { x: 0.0, y: 1.0 };
            game.running = true;
            unsafe {
                ALLOW_MOVE = false;
            }
        }

        if unsafe { IsKeyPressed(KeyboardKey::KEY_LEFT as c_int) }
            && game.snake.direction.x != 1.0
            && unsafe { ALLOW_MOVE }
        {
            game.snake.direction = Vector2 { x: -1.0, y: 0.0 };
            game.running = true;
            unsafe {
                ALLOW_MOVE = false;
            }
        }

        if unsafe { IsKeyPressed(KeyboardKey::KEY_RIGHT as c_int) }
            && game.snake.direction.x != -1.0
            && unsafe { ALLOW_MOVE }
        {
            game.snake.direction = Vector2 { x: 1.0, y: 0.0 };
            game.running = true;
            unsafe {
                ALLOW_MOVE = false;
            }
        }

        // Drawing
        unsafe {
            ClearBackground(GREEN);
            DrawRectangleLinesEx(
                Rectangle {
                    x: OFFSET - 5.0,
                    y: OFFSET - 5.0,
                    width: CELL_SIZE * CELL_COUNT + 10.0,
                    height: CELL_SIZE * CELL_COUNT + 10.0,
                },
                5,
                DARK_GREEN,
            );
            {
                let text = CString::new("Retro Snake").unwrap();
                DrawText(text.as_ptr(), (OFFSET - 5.0) as c_int, 20, 40, DARK_GREEN);
            }
            {
                let text = CString::new(format!("{}", game.score)).unwrap();
                DrawText(
                    text.as_ptr(),
                    (OFFSET - 5.0) as c_int,
                    (OFFSET + CELL_SIZE * CELL_COUNT + 10.0) as c_int,
                    40,
                    DARK_GREEN,
                );
            }
            game.draw();

            EndDrawing();
        }
    }

    unsafe {
        CloseWindow();
    }
}
