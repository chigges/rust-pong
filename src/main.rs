use macroquad::prelude::*;


enum Player {
    TopPlayer = 0,
    BottomPlayer,
}


struct MainState {
    ball: Rect,
    ball_vel: Vec2,

    top_paddle: Rect,
    bottom_paddle: Rect,

    top_paddle_vel: Vec2,
    bottom_paddle_vel: Vec2,

    top_player_score: i32,
    bottom_player_score: i32,

    most_recent_scorer: Player,
}



impl MainState {
    fn reset_ball(&mut self) {
        self.ball = Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0);

        // TODO: Randomly send to left or right
        // TODO: Have it so player decides when the ball launches or the ball is slow until hits
        // paddle first time (give time to react)
        if matches!(self.most_recent_scorer, Player::TopPlayer) {
            self.ball_vel = Vec2::new(1.0, -2.0);
        }
        else {
            self.ball_vel = Vec2::new(1.0, 2.0);
        }
    }


    pub fn update(&mut self) {
        // Ball movement
        self.ball.move_to(Vec2::new(
                self.ball.x + self.ball_vel.x,
                self.ball.y + self.ball_vel.y,
        ));

        let ball_collision = self.ball.overlaps(&self.top_paddle)    && self.ball_vel.y < 0.0 ||
                             self.ball.overlaps(&self.bottom_paddle) && self.ball_vel.y > 0.0;

        // Controlling paddles
        const PADDLE_SPEED: f32 = 5.0;
        if is_key_down(KeyCode::A) && self.top_paddle.left() > 0.0 {
            self.top_paddle_vel.x = -PADDLE_SPEED;
        }
        else if is_key_down(KeyCode::S) && self.top_paddle.right() < screen_width() {
            self.top_paddle_vel.x = PADDLE_SPEED;
        }
        else {
            self.top_paddle_vel.x = 0.0;
        }
        self.top_paddle.x += self.top_paddle_vel.x;

        if is_key_down(KeyCode::Left) && self.bottom_paddle.left() > 0.0 {
            self.bottom_paddle_vel.x = -PADDLE_SPEED;
        }
        else if is_key_down(KeyCode::Right) && self.bottom_paddle.right() < screen_width() {
            self.bottom_paddle_vel.x = PADDLE_SPEED;
        }
        else {
            self.bottom_paddle_vel.x = 0.0;
        }
        self.bottom_paddle.x += self.bottom_paddle_vel.x;


        // Ball bounce off paddles
        if ball_collision {
            self.ball_vel.y *= -1.0;

            // TODO: Empart a bit of random velocity to the ball
            const EMPARTED_PADDLE_VELOCITY: f32 = 4.0;
            let is_on_bottom_half_of_screen = self.ball.y > screen_height() / 2.0;
            if is_on_bottom_half_of_screen && self.bottom_paddle_vel.x > 0.0 {
                self.ball_vel.x += EMPARTED_PADDLE_VELOCITY;
            }
            else if is_on_bottom_half_of_screen && self.bottom_paddle_vel.x < 0.0 {
                self.ball_vel.x -= EMPARTED_PADDLE_VELOCITY;
            }
            else if !is_on_bottom_half_of_screen && self.top_paddle_vel.x > 0.0 {
                self.ball_vel.x += EMPARTED_PADDLE_VELOCITY;
            }
            else if !is_on_bottom_half_of_screen && self.top_paddle_vel.x < 0.0 {
                self.ball_vel.x -= EMPARTED_PADDLE_VELOCITY;
            }

            // Randomness!
            // const RANDOM_VELOCITY_X: f32 = 0.75;
            // const RANDOM_VELOCITY_Y: f32 = 0.5;
            // self.ball_vel.x += rand::gen_range(-RANDOM_VELOCITY_X, RANDOM_VELOCITY_X);
            // self.ball_vel.y += rand::gen_range(-RANDOM_VELOCITY_Y, RANDOM_VELOCITY_Y);

            if self.ball_vel.y == 0.0 {
                self.ball_vel.y = 0.5
            }
        }

        // Bounce ball off sides of screen
        if self.ball.right() >= screen_width() && self.ball_vel.x > 0.0 ||
           self.ball.left()  <= 0.0            && self.ball_vel.x < 0.0 {
               self.ball_vel.x *= -1.0;
        }

        // Scoring
        if self.ball.bottom() >= screen_height() && self.ball_vel.y > 0.0 {
            self.top_player_score += 1;
            self.most_recent_scorer = Player::BottomPlayer;
            self.reset_ball();
        }
        if self.ball.top() <= 0.0 && self.ball_vel.y < 0.0 {
            self.bottom_player_score += 1;
            self.most_recent_scorer = Player::TopPlayer;
            self.reset_ball();
        }
    }


    fn draw_rect_object(&mut self, rect: Rect) {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
    }


    pub fn draw(&mut self) {
        self.draw_rect_object(self.ball);
        self.draw_rect_object(self.top_paddle);
        self.draw_rect_object(self.bottom_paddle);

        const VERTICAL_MARGIN: f32 = 50.0;
        const HORIZONTAL_MARGIN: f32 = 20.0;
        const FONT_SIZE: f32 = 40.0;
        draw_text(&self.top_player_score.to_string(), HORIZONTAL_MARGIN, VERTICAL_MARGIN + (FONT_SIZE / 2.0), FONT_SIZE, WHITE);
        draw_text(&self.bottom_player_score.to_string(), HORIZONTAL_MARGIN, screen_height() - VERTICAL_MARGIN, FONT_SIZE, WHITE);
    }
}


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
