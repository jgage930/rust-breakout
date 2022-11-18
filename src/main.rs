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
pub const PADDLE_SPEED: f32 = 10.0; 


struct MainState {
    paddle: Paddle,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            paddle: Paddle::new()?,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;

        if k_ctx.is_key_pressed(KeyCode::A) {
            if self.paddle.rect.x - PADDLE_SPEED >= 0.0 {
                self.paddle.rect.x -= PADDLE_SPEED;
            }
        }

        if k_ctx.is_key_pressed(KeyCode::D) {
            if self.paddle.rect.x + PADDLE_SPEED + PADDLE_W <= WIN_SIZE.0 {
                self.paddle.rect.x += PADDLE_SPEED;
            }
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let paddle_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), self.paddle.rect, Color::WHITE)?;

        canvas.draw(&paddle_mesh, DrawParam::default());

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

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Breakout", "jack")
        .window_setup(conf::WindowSetup::default().title("Breakout"))
        .window_mode(conf::WindowMode::default().dimensions(WIN_SIZE.0, WIN_SIZE.1));

    let (mut ctx, event_loop) = cb.build()?;
    // let state = MainState::new(&mut ctx)?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
