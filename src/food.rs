use std::{
    collections::VecDeque,
    ffi::{c_int, CString},
};

use raylib::{
    ffi::{
        DrawTexture, GetRandomValue, LoadImage, LoadTextureFromImage, Texture, UnloadImage,
        UnloadTexture,
    },
    math::Vector2,
};

use crate::{utils::element_in_deque, CELL_COUNT, CELL_SIZE, OFFSET};

#[derive(Debug)]
pub struct Food {
    pub position: Vector2,
    pub texture: Texture,
}

impl Food {
    pub fn new(snake_body: &VecDeque<Vector2>) -> Food {
        let image_path = CString::new("graphics/food.png").unwrap();
        let image = unsafe { LoadImage(image_path.as_ptr()) };
        let texture = unsafe { LoadTextureFromImage(image) };
        unsafe {
            UnloadImage(image);
        }
        let position = Self::generate_random_pos(snake_body);

        Food { position, texture }
    }

    pub fn draw(&self) {
        unsafe {
            DrawTexture(
                self.texture,
                (OFFSET + self.position.x * CELL_SIZE) as c_int,
                (OFFSET + self.position.y * CELL_SIZE) as c_int,
                raylib::color::Color::WHITE.into(),
            );
        }
    }

    pub fn generate_random_cell() -> Vector2 {
        let x = unsafe { GetRandomValue(0, CELL_COUNT as c_int - 1) } as f32;
        let y = unsafe { GetRandomValue(0, CELL_COUNT as c_int - 1) } as f32;
        Vector2 { x, y }
    }

    pub fn generate_random_pos(snake_body: &VecDeque<Vector2>) -> Vector2 {
        let mut position = Self::generate_random_cell();

        while element_in_deque(position, snake_body) {
            position = Self::generate_random_cell();
        }
        position
    }
}

impl Drop for Food {
    fn drop(&mut self) {
        unsafe {
            UnloadTexture(self.texture);
        }
    }
}
