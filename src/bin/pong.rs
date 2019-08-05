use ggez::event;
use ggez::graphics;
use ggez::nalgebra;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState{ pos_x: 0.0})
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = (self.pos_x + 1.0) % 800.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.2, 0.5, 0.7, 1.0].into());

        let circle = graphics::Mesh::new_circle(ctx,
        graphics::DrawMode::fill(), nalgebra::Point2::new(0.0, 0.0), 100.0, 2.0, graphics::WHITE)?;
        graphics::draw(ctx, &circle, (nalgebra::Point2::new(self.pos_x, 380.0),))?;
        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("simple_pong", "shessel");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
