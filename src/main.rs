use ggez::{
    Context,
    GameResult,
    event,
    graphics::{Canvas, Color},
    timer,
    conf
};

pub const WIN_SIZE: (f32, f32) = (400.0, 600.0);


struct MainState {}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {

        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = Canvas::from_frame(ctx, Color::BLACK);

        canvas.finish(ctx)?;
        timer::yield_now();

        Ok(())
    }
}



fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Space_Race", "jack")
        .window_setup(conf::WindowSetup::default().title("Space Race"))
        .window_mode(conf::WindowMode::default().dimensions(WIN_SIZE.0, WIN_SIZE.1));

    let (mut ctx, event_loop) = cb.build()?;
    // let state = MainState::new(&mut ctx)?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
