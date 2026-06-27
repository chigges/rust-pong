use glam::Vec2;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect {
            x, y, w, h,
        }
    }
    fn left(&self) -> f32 {
        self.x
    }

    fn right(&self) -> f32 {
        self.x + self.w
    }

    fn top(&self) -> f32 {
        self.y
    }

    fn bottom(&self) -> f32 {
        self.y + self.h
    }

    fn overlaps(&self, other: &Rect) -> bool {
        self.left() < other.right() &&
        self.right() > other.left() &&
        self.top() < other.bottom() &&
        self.bottom() > other.top()
    }

    fn move_to(&mut self, destination: Vec2) {
        self.x = destination.x;
        self.y = destination.y;
    }
}

pub enum PlayerInput {
    None = 0,
    Left,
    Right,
}

enum Player {
    TopPlayer = 0,
    BottomPlayer,
}

pub struct Game {
    pub screen_width: f32,
    pub screen_height: f32,

    pub ball: Rect,
    pub ball_vel: Vec2,

    pub top_paddle: Rect,
    pub bottom_paddle: Rect,

    pub top_paddle_vel: Vec2,
    pub bottom_paddle_vel: Vec2,

    pub top_player_score: i32,
    pub bottom_player_score: i32,

    pub most_recent_scorer: Player,
}

impl Game {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            screen_width: width,
            screen_height: height,

            ball: Rect::new(width / 2.0, height / 2.0, 10.0, 10.0),
            ball_vel: Vec2::new(1.0, -2.0),

            top_paddle: Rect::new(width / 2.0, 20.0, 100.0, 5.0),
            top_paddle_vel: Vec2::new(0.0, 0.0),
            top_player_score: 0,

            bottom_paddle: Rect::new(width / 2.0, height - 20.0, 100.0, 5.0),
            bottom_paddle_vel: Vec2::new(0.0, 0.0),
            bottom_player_score: 0,

            most_recent_scorer: Player::BottomPlayer,
        }
    }
    
    fn reset_ball(&mut self) {
        self.ball = Rect::new(self.screen_width / 2.0, self.screen_height / 2.0, 10.0, 10.0);

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


    pub fn update(&mut self, input1: PlayerInput, input2: PlayerInput) {
        // Ball movement
        self.ball.move_to(Vec2::new(
                self.ball.x + self.ball_vel.x,
                self.ball.y + self.ball_vel.y,
        ));

        let ball_collision = self.ball.overlaps(&self.top_paddle)    && self.ball_vel.y < 0.0 ||
                             self.ball.overlaps(&self.bottom_paddle) && self.ball_vel.y > 0.0;

        // Controlling paddles
        const PADDLE_SPEED: f32 = 5.0;
        if matches!(input1, PlayerInput::Left) && self.top_paddle.left() > 0.0 {
            self.top_paddle_vel.x = -PADDLE_SPEED;
        }
        else if matches!(input1, PlayerInput::Right) && self.top_paddle.right() < self.screen_width {
            self.top_paddle_vel.x = PADDLE_SPEED;
        }
        else {
            self.top_paddle_vel.x = 0.0;
        }
        self.top_paddle.x += self.top_paddle_vel.x;

        if matches!(input2, PlayerInput::Left) && self.bottom_paddle.left() > 0.0 {
            self.bottom_paddle_vel.x = -PADDLE_SPEED;
        }
        else if matches!(input2, PlayerInput::Right) && self.bottom_paddle.right() < self.screen_width {
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
            let is_on_bottom_half_of_screen = self.ball.y > self.screen_height / 2.0;
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
        if self.ball.right() >= self.screen_width && self.ball_vel.x > 0.0 ||
           self.ball.left()  <= 0.0            && self.ball_vel.x < 0.0 {
               self.ball_vel.x *= -1.0;
        }

        // Scoring
        if self.ball.bottom() >= self.screen_height && self.ball_vel.y > 0.0 {
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
}

