use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use ggez::event::{KeyCode};

use std::env;
use std::path;
use std::vec;

const WINDOW_SIZE: (f32, f32) = (800.0, 800.0);
const ENEMY_WIDTH: f32 = 40.0;

struct Ball {
    image: graphics::Image,
    pos_x: f32,
    pos_y: f32,
    speed: f32,
    dir_x: i8,
    dir_y: i8,
    reached_bottom: bool,
}

impl Ball {
    fn new(ctx: &mut Context, x: f32, y: f32) -> Ball {
        Ball {
            image: graphics::Image::new(ctx, "/ballis.png").unwrap(),
            pos_x: x,
            pos_y: y,
            speed: 2.5,
            dir_x: 0,
            dir_y: 1,
            reached_bottom: false,
        }
    }

    fn has_reached_bottom(&mut self) -> bool {
        self.pos_y + f32::from(self.image.height()) > WINDOW_SIZE.1
    }

    fn has_reached_top(&mut self) -> bool {
        self.pos_y <= 0.0
    }

    fn hit_right_wall(&mut self) -> bool {
        self.pos_x <= 0.0
    }

    fn hit_left_wall(&mut self) -> bool {
        self.pos_x + self.image.width() as f32 >= WINDOW_SIZE.0
    }

    fn hit_paddle(&mut self, paddle: &Paddle) -> bool {
        self.pos_y + f32::from(self.image.height()) > paddle.pos_y &&
            self.pos_y + f32::from(self.image.height()) < paddle.pos_y + paddle.height &&
            (self.pos_x + f32::from(self.image.width()) > paddle.pos_x &&
             self.pos_x + f32::from(self.image.width()) < paddle.pos_x + paddle.length ||
             self.pos_x > paddle.pos_x && self.pos_x < paddle.pos_x + paddle.length)
    }

    fn paddle_side_hit(&mut self, paddle: &Paddle) -> i8 {
        if self.pos_x + f32::from(self.image.width() / 2) < paddle.pos_x + paddle.length / 2.0 {
            -1
        } else {
            1
        }
    }
}

struct Paddle {
    pos_x: f32,
    pos_y: f32,
    speed: f32,
    length: f32,
    height: f32,
    rec: graphics::Rect,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Paddle {
        Paddle {
            pos_x: x,
            pos_y: y,
            speed: 5.0,
            length: 50.0,
            height: 10.0,
            rec: graphics::Rect::new(0.0, 0.0, 50.0, 10.0),
        }
    }

    fn hit_left_wall(&mut self) -> bool {
        self.pos_x - self.speed >= 0.0
    }

    fn hit_right_wall(&mut self) -> bool{
        self.pos_x + self.length + self.speed <= WINDOW_SIZE.0
    }
}

struct Enemy {
    health: u8,
    pos_x: f32,
    pos_y: f32,
}

impl Enemy {
    fn new(ctx: &mut Context, h: u8, x: f32, y: f32) -> Enemy {
        Enemy {
            health: h,
            pos_x: x,
            pos_y: y,
        }
    }
}

struct MainState {
    paddle: Paddle,
    balls: Vec<Ball>,
    enemies: Vec<Enemy>,
    sb: graphics::spritebatch::SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // TODO: replace 25 by paddle length / 2
        let paddle = Paddle::new(WINDOW_SIZE.0 / 2.0 - 25.0, WINDOW_SIZE.1 - 50.0);

        let balls = vec![
            Ball::new(ctx, 100.0, 100.0),
            Ball::new(ctx, 200.0, 200.0),
            Ball::new(ctx, 300.0, 300.0),
            Ball::new(ctx, 400.0, 400.0),
            Ball::new(ctx, 500.0, 500.0),
        ];

        let enemy_image = graphics::Image::new(ctx, "/enemy.png").unwrap();
        let mut sb = graphics::spritebatch::SpriteBatch::new(enemy_image);

        let mut enemies = Vec::new();

        for i in 1..5 {
            let next_y_pos = 20.0 * i as f32;
            for i in 1..15 {
                let next_x_pos = (ENEMY_WIDTH + 10.0) * i as f32;
                enemies.push(Enemy::new(ctx, 1, next_x_pos, next_y_pos));

                let p = graphics::DrawParam::new()
                    .dest(Point2::new(next_x_pos, next_y_pos));
                sb.add(p);
            }
        }

        let s = MainState {
            paddle,
            balls,
            enemies,
            sb,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.balls.retain(|ball| !ball.reached_bottom);

        for ball in &mut self.balls {
            if ball.has_reached_bottom() {
                ball.reached_bottom = true;
            } else if ball.has_reached_top() {
                ball.dir_y = 1;
            } else if ball.hit_right_wall() {
                ball.dir_x = 1
            } else if ball.hit_left_wall() {
                ball.dir_x = -1
            } else if ball.hit_paddle(&self.paddle) {
                ball.dir_x = ball.paddle_side_hit(&self.paddle);
                ball.dir_y = -1;
            }
            ball.pos_x = ball.pos_x % WINDOW_SIZE.0 + ball.speed * f32::from(ball.dir_x);
            ball.pos_y = ball.pos_y % WINDOW_SIZE.1 + ball.speed * f32::from(ball.dir_y);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if self.paddle.hit_left_wall() {
                self.paddle.pos_x = self.paddle.pos_x % graphics::size(ctx).0 - self.paddle.speed;
            }
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if self.paddle.hit_right_wall() {
                self.paddle.pos_x = self.paddle.pos_x % graphics::size(ctx).0 + self.paddle.speed;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for ball in &self.balls {
            graphics::draw(ctx, &ball.image, (Point2::new(ball.pos_x, ball.pos_y),))?;
        }

        graphics::draw(ctx, &self.sb, graphics::DrawParam::new().dest(Point2::new(0.0, 0.0)))?;

        let paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.paddle.rec,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        graphics::draw(ctx, &paddle_mesh,
                       (Point2::new(self.paddle.pos_x, self.paddle.pos_y),)
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
