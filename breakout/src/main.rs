use macroquad::prelude::*;

mod ball;
mod block;
mod collisions;
mod player;

use ball::Ball;
use block::{Block, BLOCK_SIZE};
use player::Player;

use crate::collisions::resolve_collision;

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();

    let mut blocks = Vec::<Block>::new();
    let mut balls = Vec::<Ball>::new();

    let (width, height) = (6, 6);

    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );

    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / height) as f32 * total_block_size.y;

        blocks.push(Block::new(
            board_start_pos + const_vec2!([block_x, block_y]),
        ));
    }

    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32,
        screen_height() * 0.5f32,
    )));

    loop {
        player.update(get_frame_time());
        balls
            .iter_mut()
            .for_each(|ball| ball.update(get_frame_time()));

        balls.iter_mut().for_each(|ball| {
            resolve_collision(&mut ball.rect, &player.rect, &mut ball.vel);

            blocks.iter_mut().for_each(|block| {
                if resolve_collision(&mut ball.rect, &block.rect, &mut ball.vel) {
                    block.lose_life();
                }
            })
        });

        blocks.retain(|block| block.lives != 0);

        clear_background(WHITE);

        blocks.iter().for_each(|block| block.draw());

        balls.iter().for_each(|ball| ball.draw());
        player.draw();
        next_frame().await
    }
}
