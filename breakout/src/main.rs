use macroquad::prelude::*;

mod ball;
mod block;
mod collisions;
mod game_states;
mod player;
mod title_text;

use ball::Ball;
use block::{Block, BLOCK_SIZE};
use player::Player;

use crate::{collisions::resolve_collision, game_states::GameState, title_text::draw_title_text};

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/ComicMonoNF.ttf").await.unwrap();

    let mut game_state = GameState::Menu;
    let mut player = Player::new();

    let mut blocks = Vec::<Block>::new();
    let mut balls = Vec::<Ball>::new();

    let mut score = 0;
    let mut lives = 3;

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
        screen_height() * 0.6f32,
    )));

    loop {
        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Game => {
                if is_key_pressed(KeyCode::Space) && balls.is_empty() {
                    balls.push(Ball::new(vec2(
                        screen_width() * 0.5f32,
                        screen_height() * 0.6f32,
                    )));
                }

                player.update(get_frame_time());
                balls
                    .iter_mut()
                    .for_each(|ball| ball.update(get_frame_time()));

                balls.iter_mut().for_each(|ball| {
                    resolve_collision(&mut ball.rect, &player.rect, &mut ball.vel);

                    blocks.iter_mut().for_each(|block| {
                        if resolve_collision(&mut ball.rect, &block.rect, &mut ball.vel) {
                            block.lose_life();
                            score += 10;
                        }
                    })
                });

                blocks.retain(|block| block.lives != 0);

                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }

                let ball_length = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());

                let has_lost_balls = ball_length - balls.len() > 0;

                if has_lost_balls && balls.is_empty() {
                    lives -= 1;

                    if lives == 0 {
                        game_state = GameState::Dead;
                    }
                };
            }

            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                }
            }
        }

        clear_background(WHITE);

        blocks.iter().for_each(|block| block.draw());

        balls.iter().for_each(|ball| ball.draw());
        player.draw();

        match game_state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start", font);
            }
            GameState::Game => {
                let score_text = &format!("Score: {}", score);
                let score_width = measure_text(score_text, Some(font), 30u16, 1.0);

                let lives_text = &format!("Lives: {}", lives);

                draw_text_ex(
                    score_text,
                    (screen_width() - score_width.width) * 0.5f32,
                    40f32,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    lives_text,
                    50f32,
                    40f32,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::LevelCompleted => draw_title_text(&format!("You win: {}", score), font),
            GameState::Dead => draw_title_text(&format!("You lost: {}", score), font),
        }
        next_frame().await
    }
}
