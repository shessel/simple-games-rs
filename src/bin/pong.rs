use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

struct BallState {
    pos: Point2<f32>,
    vel: Vector2<f32>,
}

#[derive(Clone, Copy)]
struct PaddleState {
    pos: Point2<f32>,
}

#[repr(usize)]
enum PaddlePosition {
    Left,
    Right,
    Count,
}

struct MainState {
    ball: BallState,
    paddles: [PaddleState; PaddlePosition::Count as usize],
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let ball = BallState {
            pos: Point2::new(0.0, 300.0),
            vel: Vector2::new(4.0, 4.0),
        };

        let mut paddles = [PaddleState {
            pos: Point2::new(0.0, 0.0),
        }; PaddlePosition::Count as usize];

        paddles[PaddlePosition::Left as usize] = PaddleState {
            pos: Point2::new(10.0, 300.0),
        };

        paddles[PaddlePosition::Right as usize] = PaddleState {
            pos: Point2::new(790.0, 300.0),
        };

        let main_state = MainState { ball, paddles };
        Ok(main_state)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            for paddle in self.paddles.iter_mut() {
                paddle.pos.y -= 4.0;
            }
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            for paddle in self.paddles.iter_mut() {
                paddle.pos.y += 4.0;
            }
        }

        if self.ball.vel.x > 0.0 {
            if (self.ball.pos.x - self.paddles[PaddlePosition::Right as usize].pos.x).abs() < 5.0
                && (self.ball.pos.y - self.paddles[PaddlePosition::Right as usize].pos.y).abs()
                    < 50.0
            {
                self.ball.vel.x *= -1.0;
            } else if self.ball.pos.x > self.paddles[PaddlePosition::Right as usize].pos.x + 5.0 {
                self.ball.vel.x *= -1.0;
                self.ball.pos.x = 400.0;
            }
        } else {
            if (self.ball.pos.x - self.paddles[PaddlePosition::Left as usize].pos.x).abs() < 5.0
                && (self.ball.pos.y - self.paddles[PaddlePosition::Left as usize].pos.y).abs()
                    < 50.0
            {
                self.ball.vel.x *= -1.0;
            } else if self.ball.pos.x < self.paddles[PaddlePosition::Left as usize].pos.x - 5.0 {
                self.ball.vel.x *= -1.0;
                self.ball.pos.x = 400.0;
            }
        }
        if self.ball.pos.y < 0.0 && self.ball.vel.y < 0.0
            || self.ball.pos.y > 600.0 && self.ball.vel.y > 0.0
        {
            self.ball.vel.y *= -1.0;
        }
        self.ball.pos += self.ball.vel;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.2, 0.5, 0.7, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            10.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (self.ball.pos,))?;

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-5.0, -50.0, 10.0, 100.0),
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, (self.paddles[0].pos,))?;
        graphics::draw(ctx, &rect, (self.paddles[1].pos,))?;
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
