use std::collections::VecDeque;

use raylib::{
    ffi::{DrawRectangleRounded, Rectangle},
    math::Vector2,
};

use crate::{colors::DARK_GREEN, utils::deque, CELL_SIZE, OFFSET};

#[derive(Debug, Default)]
pub struct Snake {
    pub body: VecDeque<Vector2>,
    pub direction: Vector2,
    pub add_segment: bool,
}

impl Snake {
    pub fn new() -> Self {
        let body = deque![
            Vector2 { x: 6.0, y: 9.0 },
            Vector2 { x: 5.0, y: 9.0 },
            Vector2 { x: 4.0, y: 9.0 }
        ];
        let direction = Vector2 { x: 1.0, y: 0.0 };
        let add_segment = false;

        Self {
            body,
            direction,
            add_segment,
        }
    }

    pub fn draw(&self) {
        for i in 0..self.body.len() {
            let x = self.body[i].x;
            let y = self.body[i].y;
            let segment = Rectangle {
                x: OFFSET + x * CELL_SIZE,
                y: OFFSET + y * CELL_SIZE,
                width: CELL_SIZE,
                height: CELL_SIZE,
            };
            unsafe {
                DrawRectangleRounded(segment, 0.5, 6, DARK_GREEN);
            }
        }
    }

    pub fn update(&mut self) {
        self.body.push_front(self.body[0] + self.direction);
        if self.add_segment {
            self.add_segment = false;
        } else {
            self.body.pop_back();
        }
    }

    pub fn reset(&mut self) {
        self.body = deque![
            Vector2 { x: 6.0, y: 9.0 },
            Vector2 { x: 5.0, y: 9.0 },
            Vector2 { x: 4.0, y: 9.0 }
        ];
        self.direction = Vector2 { x: 1.0, y: 0.0 }
    }
}
