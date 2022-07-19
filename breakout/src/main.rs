use macroquad::prelude::*;

const BALL_SIZE: f32 = 20f32;
const BALL_SPEED: f32 = 200f32;
const BLOCK_SIZE: Vec2 = const_vec2!([80f32, 40f32]);
const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;


pub struct Ball {
    rect: Rect,
    vel: Vec2,
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

struct Block {
    rect: Rect,
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

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (
            is_key_down(KeyCode::Left),
            is_key_down(KeyCode::Right)
        ) {
                (true, false) => -1f32,
                (false, true) => 1f32,
                _ => 0f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        let rightest_x: f32 = screen_width() - self.rect.w;
        if self.rect.x > rightest_x {
            self.rect.x = rightest_x;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            BLUE,
        );
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();

    let mut blocks = Vec::<Block>::new();
    let mut balls = Vec::<Ball>::new();


    let (width, height) = ( 6, 6);


    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / height) as f32 * total_block_size.y;

        blocks.push(Block::new(board_start_pos + const_vec2!([block_x, block_y])));


    }

    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));



    loop {
        player.update(get_frame_time());
        balls.iter_mut()
            .for_each(|ball| { 
                ball.update(get_frame_time())
            });


        clear_background(WHITE);

        blocks.iter()
            .for_each(|block| block.draw());

        balls.iter()
            .for_each(|ball| ball.draw());
        player.draw();
        next_frame().await
    }
}
