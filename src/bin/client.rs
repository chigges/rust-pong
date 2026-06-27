use macroquad::prelude::*;
use pong::game::{Game, PlayerInput};


fn draw_rect_object(rect: Rect) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
}


fn draw(game: &Game) {
    draw_rect_object(Rect::new(game.ball.x, game.ball.y, game.ball.w, game.ball.h));
    draw_rect_object(Rect::new(game.bottom_paddle.x, game.bottom_paddle.y, game.bottom_paddle.w, game.bottom_paddle.h));
    draw_rect_object(Rect::new(game.top_paddle.x, game.top_paddle.y, game.top_paddle.w, game.top_paddle.h));

    const VERTICAL_MARGIN: f32 = 50.0;
    const HORIZONTAL_MARGIN: f32 = 20.0;
    const FONT_SIZE: f32 = 40.0;
    draw_text(&game.top_player_score.to_string(), HORIZONTAL_MARGIN, VERTICAL_MARGIN + (FONT_SIZE / 2.0), FONT_SIZE, WHITE);
    draw_text(&game.bottom_player_score.to_string(), HORIZONTAL_MARGIN, screen_height() - VERTICAL_MARGIN, FONT_SIZE, WHITE);
}


#[macroquad::main("Pong")]
async fn main() {
    println!("Client running...");
    let mut game = Game::new(screen_width(), screen_height());

    loop {
        clear_background(BLACK);

        let mut input1: PlayerInput = PlayerInput::None;
        if is_key_down(KeyCode::Left) {
            input1 = PlayerInput::Left;
        }
        if is_key_down(KeyCode::Right) {
            input1 = PlayerInput::Right;
        }

        let mut input2: PlayerInput = PlayerInput::None;
        if is_key_down(KeyCode::A) {
            input2 = PlayerInput::Left;
        }
        if is_key_down(KeyCode::D) {
            input2 = PlayerInput::Right;
        }

        game.update(input1, input2);
        draw(&game);

        next_frame().await
    }
}


/*
pub mod state;
pub mod game;

use macroquad::prelude::*;



#[macroquad::main("Pong")]
async fn main() {
    let mut state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        ball_vel: Vec2::new(1.0, 2.0),
        top_paddle: Rect::new(screen_width() / 2.0, 20.0, 100.0, 5.0), bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 20.0, 100.0, 5.0), top_player_score: 0,
        top_paddle_vel: Vec2::new(0.0, 0.0), bottom_paddle_vel: Vec2::new(0.0, 0.0),
        bottom_player_score: 0,
        most_recent_scorer: Player::BottomPlayer,
    };

    loop {
        clear_background(BLACK);

        state.update();
        state.draw();

        next_frame().await
    }
}






*/
