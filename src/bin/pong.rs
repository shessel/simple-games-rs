use std::f32;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Align, DrawMode, Font, Mesh, Rect, Scale, Text};
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
#[derive(PartialEq, Clone, Copy)]
enum Player {
    Left,
    Right,
    Count,
}

#[derive(PartialEq)]
enum Phase {
    Paused,
    Playing,
    Serve(Player),
}

struct MainState {
    ball: BallState,
    paddles: [PaddleState; Player::Count as usize],
    score: [u8; Player::Count as usize],
    phase: Phase,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let ball = BallState {
            pos: Point2::new(400.0, 300.0),
            vel: Vector2::new(4.0, 4.0),
        };

        let mut paddles = [PaddleState {
            pos: Point2::new(0.0, 0.0),
        }; Player::Count as usize];

        paddles[Player::Left as usize] = PaddleState {
            pos: Point2::new(10.0, 300.0),
        };

        paddles[Player::Right as usize] = PaddleState {
            pos: Point2::new(790.0, 300.0),
        };

        let score = [0u8; Player::Count as usize];

        let main_state = MainState {
            ball,
            paddles,
            score,
            phase: Phase::Paused,
        };
        Ok(main_state)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.phase == Phase::Paused {
            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.phase = Phase::Playing;
            }
        } else {
            if keyboard::is_key_pressed(ctx, KeyCode::Up) {
                self.paddles[Player::Right as usize].pos.y -= 4.0;
                self.paddles[Player::Right as usize].pos.y =
                    self.paddles[Player::Right as usize].pos.y.max(60.0);
            } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
                self.paddles[Player::Right as usize].pos.y += 4.0;
                self.paddles[Player::Right as usize].pos.y =
                    self.paddles[Player::Right as usize].pos.y.min(540.0);
            }

            if keyboard::is_key_pressed(ctx, KeyCode::W) {
                self.paddles[Player::Left as usize].pos.y -= 4.0;
                self.paddles[Player::Left as usize].pos.y =
                    self.paddles[Player::Left as usize].pos.y.max(60.0);
            } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
                self.paddles[Player::Left as usize].pos.y += 4.0;
                self.paddles[Player::Left as usize].pos.y =
                    self.paddles[Player::Left as usize].pos.y.min(540.0);
            }

            if let Phase::Serve(player) = &self.phase {
                self.ball.pos.y = self.paddles[*player as usize].pos.y;
                if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                    self.phase = Phase::Playing;
                }
            } else {
                if self.ball.vel.x > 0.0 {
                    if self.paddles[Player::Right as usize].pos.x - self.ball.pos.x < 10.0
                        && (self.ball.pos.y - self.paddles[Player::Right as usize].pos.y).abs()
                            < 55.0
                    {
                        self.ball.vel.x *= -1.0;
                    } else if self.ball.pos.x > self.paddles[Player::Right as usize].pos.x + 10.0 {
                        self.ball.vel.x *= -1.0;
                        self.ball.pos = self.paddles[Player::Right as usize].pos;
                        self.ball.pos.x -= 15.0;
                        self.score[Player::Left as usize] += 1;
                        self.phase = Phase::Serve(Player::Right);
                    }
                } else {
                    if self.ball.pos.x - self.paddles[Player::Left as usize].pos.x < 10.0
                        && (self.ball.pos.y - self.paddles[Player::Left as usize].pos.y).abs()
                            < 55.0
                    {
                        self.ball.vel.x *= -1.0;
                    } else if self.ball.pos.x < self.paddles[Player::Left as usize].pos.x - 10.0 {
                        self.ball.vel.x *= -1.0;
                        self.ball.pos = self.paddles[Player::Left as usize].pos;
                        self.ball.pos.x += 15.0;
                        self.score[Player::Right as usize] += 1;
                        self.phase = Phase::Serve(Player::Left);
                    }
                }
                if self.ball.pos.y < 5.0 && self.ball.vel.y < 0.0
                    || self.ball.pos.y > 595.0 && self.ball.vel.y > 0.0
                {
                    self.ball.vel.y *= -1.0;
                }

                if self.phase == Phase::Playing {
                    self.ball.pos += self.ball.vel;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.2, 0.5, 0.7, 1.0].into());

        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            10.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (self.ball.pos,))?;

        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(-5.0, -50.0, 10.0, 100.0),
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, (self.paddles[0].pos,))?;
        graphics::draw(ctx, &rect, (self.paddles[1].pos,))?;

        let mut score_text = Text::new(format!(
            "{} : {}",
            self.score[Player::Left as usize],
            self.score[Player::Right as usize]
        ));
        score_text.set_font(Font::default(), Scale::uniform(25.0));
        score_text.set_bounds(Point2::new(800.0, 100.0), Align::Center);

        graphics::draw(ctx, &score_text, (Point2::new(0.0, 0.0), graphics::WHITE))?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("simple_pong", "shessel")
        .window_setup(WindowSetup::default().title("Pong"))
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(false),
        );
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
