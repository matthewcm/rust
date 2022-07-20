use macroquad::prelude::*;

pub const BLOCK_SIZE: Vec2 = const_vec2!([80f32, 40f32]);

pub struct Block {
    pub rect: Rect,
    pub lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                pos.x,
                pos.y,
                BLOCK_SIZE.x,
                BLOCK_SIZE.y,
            ),
            lives: 1,
        }
    }

    pub fn lose_life(&mut self) {
        self.lives -= 1;
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
