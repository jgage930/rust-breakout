use ggez::{
    Context,
    GameResult,
    event,
    graphics::{Canvas, Color, Rect, Mesh, DrawMode, DrawParam,},
    timer,
    conf,
    input::keyboard::{KeyCode,},
};

pub const WIN_SIZE: (f32, f32) = (400.0, 600.0);

pub const PADDLE_W: f32 = 75.0;
pub const PADDLE_H: f32 = 25.0;
pub const PADDLE_SPEED: f32 = 7.0; 

pub const BALL_SPEED: f32 = 5.0;
pub const BALL_SIZE: f32 = 15.0;

struct MainState {
    paddle: Paddle, ball: Ball,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            paddle: Paddle::new()?,
            ball: Ball::new()?,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Move the paddle
        let k_ctx = &ctx.keyboard;
        let mut paddle_dx = 0.0;

        if k_ctx.is_key_pressed(KeyCode::A) {
            if self.paddle.rect.x - PADDLE_SPEED >= 0.0 {
                self.paddle.rect.x -= PADDLE_SPEED;
                paddle_dx = -1.0 * PADDLE_SPEED;
            }
        }

        else if k_ctx.is_key_pressed(KeyCode::D) {
            if self.paddle.rect.x + PADDLE_SPEED + PADDLE_W <= WIN_SIZE.0 {
                self.paddle.rect.x += PADDLE_SPEED;
                paddle_dx = PADDLE_SPEED;
            }
        } else {
            paddle_dx = 0.0; 
        }

        self.paddle.rect.x += paddle_dx;


        // check if ball hits the top of screen
        if self.ball.rect.top() - BALL_SPEED <= 0.0 {
            self.ball.v_y *= -1.0;
        }

        //check if ball hits the left or right side of screen
        if self.ball.rect.left() - BALL_SPEED <= 0.0 {
            self.ball.v_x *= -1.0;
        }

        if self.ball.rect.right() + BALL_SPEED >= WIN_SIZE.0 {
            self.ball.v_x *= -1.0;
        }

        if self.ball.rect.overlaps(&self.paddle.rect) {
            self.ball.v_y *= -1.0;

            if paddle_dx < 0.0 {
                self.ball.v_x = -1.0 * BALL_SPEED;
            }

            if paddle_dx > 0.0 {
                self.ball.v_x = BALL_SPEED;
            }
        }     

        // move the ball
        self.ball.rect.y -= self.ball.v_y;
        self.ball.rect.x += self.ball.v_x;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let paddle_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), self.paddle.rect, Color::WHITE)?;
        canvas.draw(&paddle_mesh, DrawParam::default());

        let ball_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), self.ball.rect, Color::RED)?;
        canvas.draw(&ball_mesh, DrawParam::default());

        canvas.finish(ctx)?;
        timer::yield_now();

        Ok(())
    }
}

struct Paddle {
    rect: Rect,
}

impl Paddle {
    fn new() -> GameResult<Paddle> {
       Ok(
            Paddle {
                rect: Rect::new(WIN_SIZE.0 / 2.0, 550.0, PADDLE_W, PADDLE_H),
            }
       ) 
    }
    
}


struct Ball {
    v_x: f32, v_y: f32,  rect: Rect,
}

impl Ball {
    fn new() -> GameResult<Ball> {
        Ok(
            Ball {
                v_x: 0.0 , v_y: BALL_SPEED, rect: Rect::new(WIN_SIZE.0 / 2.0, WIN_SIZE.1 / 2.0, BALL_SIZE, BALL_SIZE),
            }
        )
    }
}


fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Breakout", "jack")
        .window_setup(conf::WindowSetup::default().title("Breakout"))
        .window_mode(conf::WindowMode::default().dimensions(WIN_SIZE.0, WIN_SIZE.1));

    let (mut ctx, event_loop) = cb.build()?;
    // let state = MainState::new(&mut ctx)?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
