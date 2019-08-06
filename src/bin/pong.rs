use ggez::event;
use ggez::graphics;
use ggez::nalgebra;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState{ pos_x: 0.0, pos_y: 0.0, vel_x: 4.0, vel_y: 4.0})
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x + self.vel_x;
        self.vel_x = if self.pos_x > 800.0 && self.vel_x > 0.0 
        {
            -4.0
        } else if self.pos_x < 0.0 && self.vel_x < 0.0 
        {
            4.0
        } else
        {
            self.vel_x
        };
        self.pos_y = self.pos_y + self.vel_y;
        self.vel_y = if self.pos_y > 600.0 && self.vel_y > 0.0 
        {
            -4.0
        } else if self.pos_y < 0.0 && self.vel_y < 0.0 
        {
            4.0
        } else
        {
            self.vel_y
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.2, 0.5, 0.7, 1.0].into());

        let circle = graphics::Mesh::new_circle(ctx,
        graphics::DrawMode::fill(), nalgebra::Point2::new(0.0, 0.0), 100.0, 2.0, graphics::WHITE)?;
        graphics::draw(ctx, &circle, (nalgebra::Point2::new(self.pos_x, self.pos_y),))?;
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
