use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::event::{KeyCode, KeyMods};

use std::env;
use std::path;
use std::vec;

const WINDOW_SIZE: (f32, f32) = (800.0, 800.0);

struct Ball {
    image: graphics::Image,
    pos_x: f32,
    pos_y: f32,
    speed: f32,
    dir_x: f32,
    dir_y: f32,
    reached_bottom: bool,
}

impl Ball {
    fn new(ctx: &mut Context, x: f32, y: f32) -> Ball {
        Ball {
            image: graphics::Image::new(ctx, "/ballis.png").unwrap(),
            pos_x: x,
            pos_y: y,
            speed: 2.5,
            dir_x: 1.0,
            dir_y: 0.0,
            reached_bottom: false,
        }
    }
}

struct MainState {
    paddle: graphics::Rect,
    paddle_pos_x: f32,
    paddle_pos_y: f32,
    paddle_speed: f32,
    paddle_length: f32,
    paddle_height: f32,
    balls: Vec<Ball>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let paddle_speed = 5.0;
        let paddle_length = 50.0;
        let paddle_height = 10.0;
        let paddle_pos_x = graphics::size(ctx).1 / 2.0 - paddle_length / 2.0;
        let paddle_pos_y = graphics::size(ctx).1 - 50.0;

        let paddle = graphics::Rect::new(0.0, 0.0, paddle_length, paddle_height);

        let balls = vec![
            Ball::new(ctx, 100.0, 100.0),
            Ball::new(ctx, 200.0, 200.0),
            Ball::new(ctx, 300.0, 300.0),
            Ball::new(ctx, 400.0, 400.0),
            Ball::new(ctx, 500.0, 500.0),
        ];

        let s = MainState {
            paddle,
            paddle_pos_x,
            paddle_pos_y,
            paddle_speed,
            paddle_length,
            paddle_height,
            balls,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.balls.retain(|ball| !ball.reached_bottom);

        for i in 0..self.balls.len() {
            if self.balls[i].pos_y + f32::from(self.balls[i].image.height()) > WINDOW_SIZE.1 {
                self.balls[i].reached_bottom = true;
            } else if self.balls[i].pos_y <= 0.0 {
                // Changes direction downwards
                self.balls[i].dir_x = 1.0;
            } else if self.balls[i].pos_y + f32::from(self.balls[i].image.height()) > self.paddle_pos_y &&
                self.balls[i].pos_y + f32::from(self.balls[i].image.height()) < self.paddle_pos_y + self.paddle_height &&
                (self.balls[i].pos_x + f32::from(self.balls[i].image.width()) > self.paddle_pos_x &&
                 self.balls[i].pos_x + f32::from(self.balls[i].image.width()) < self.paddle_pos_x + self.paddle_length ||
                 self.balls[i].pos_x > self.paddle_pos_x && self.balls[i].pos_x < self.paddle_pos_x + self.paddle_length) {
                self.balls[i].dir_x = -1.0;
            }

            self.balls[i].pos_y = self.balls[i].pos_y % graphics::size(ctx).1 + self.balls[i].speed * self.balls[i].dir_x;


        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if self.paddle_pos_x - self.paddle_speed >= 0.0 {
                self.paddle_pos_x = self.paddle_pos_x % graphics::size(ctx).0 - self.paddle_speed;
            }
        }
        else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if self.paddle_pos_x + self.paddle_length + self.paddle_speed <= graphics::size(ctx).0 {
                self.paddle_pos_x = self.paddle_pos_x % graphics::size(ctx).0 + self.paddle_speed;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for ball in &self.balls {
            graphics::draw(ctx, &ball.image, (Point2::new(ball.pos_x, ball.pos_y),))?;
        }

        let paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.paddle,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        graphics::draw(ctx, &paddle_mesh,
                       (Point2::new(self.paddle_pos_x, self.paddle_pos_y),)
        )?;

        graphics::present(ctx)?;
        Ok(())
    }

}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Ferris out", "Robin")
        .window_setup(ggez::conf::WindowSetup::default().title("Ferris out"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
