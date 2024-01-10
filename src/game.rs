use std::ffi::{c_int, CString};

use raylib::{
    ffi::Sound,
    ffi::{CloseAudioDevice, InitAudioDevice, LoadSound, PlaySound, UnloadSound},
};

use crate::{food::Food, snake::Snake, utils::element_in_deque, CELL_COUNT};

#[derive(Debug)]
pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub running: bool,
    pub score: c_int,
    pub eat_sound: Sound,
    pub wall_sound: Sound,
}

impl Game {
    pub fn new() -> Self {
        let snake = Snake::new();
        let food = Food::new(&snake.body);
        let running = true;
        let score = 0;

        unsafe {
            InitAudioDevice();
        }

        let eat_mp3_path = CString::new("sounds/eat.mp3").unwrap();
        let eat_sound = unsafe { LoadSound(eat_mp3_path.as_ptr()) };

        let wall_mp3_path = CString::new("sounds/wall.mp3").unwrap();
        let wall_sound = unsafe { LoadSound(wall_mp3_path.as_ptr()) };

        Self {
            snake,
            food,
            running,
            score,
            eat_sound,
            wall_sound,
        }
    }

    pub fn draw(&self) {
        self.food.draw();
        self.snake.draw();
    }

    pub fn update(&mut self) {
        if self.running {
            self.snake.update();
            self.check_collision_with_food();
            self.check_collision_with_edges();
            self.check_collision_with_tail();
        }
    }

    pub fn check_collision_with_food(&mut self) {
        if self.snake.body[0] == self.food.position {
            self.food.position = Food::generate_random_pos(&self.snake.body);
            self.snake.add_segment = true;
            self.score += 1;
            unsafe {
                PlaySound(self.eat_sound);
            }
        }
    }

    pub fn check_collision_with_edges(&mut self) {
        let error_margin = f32::EPSILON;

        if self.snake.body[0].x == CELL_COUNT || (self.snake.body[0].x - -1.0).abs() < error_margin
        {
            self.game_over();
        }

        if self.snake.body[0].y == CELL_COUNT || (self.snake.body[0].y - -1.0).abs() < error_margin
        {
            self.game_over();
        }
    }

    pub fn game_over(&mut self) {
        self.snake.reset();
        self.food.position = Food::generate_random_pos(&self.snake.body);
        self.running = false;
        self.score = 0;
        unsafe {
            PlaySound(self.wall_sound);
        }
    }

    pub fn check_collision_with_tail(&mut self) {
        let mut headless_body = self.snake.body.clone();
        headless_body.pop_front();

        if element_in_deque(self.snake.body[0], &headless_body) {
            self.game_over();
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        unsafe {
            UnloadSound(self.eat_sound);
            UnloadSound(self.wall_sound);
            CloseAudioDevice();
        }
    }
}
