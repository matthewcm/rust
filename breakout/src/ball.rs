use macroquad::prelude::*;

const BALL_SIZE: f32 = 20f32;
const BALL_SPEED: f32 = 200f32;

pub struct Ball {
    pub rect: Rect,
    pub vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32),  -1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }

        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            DARKGRAY
        )
    }
}
