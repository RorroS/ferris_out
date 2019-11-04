//! An example of how to use a `SpriteBatch`.
//!
//! You really want to run this one in release mode.

use ggez;
use ggez::event;
use ggez::graphics;
//use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
//use ggez::timer;
use ggez::{Context, GameResult};

use std::env;
use std::path;

struct MainState {
    ballis_img: graphics::Image,
    ballis_pos_y: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let ballis_img = graphics::Image::new(ctx, "/ballis.png")?;
        let ballis_pos_y = 100.0;

        let s = MainState {
            ballis_img,
            ballis_pos_y,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.ballis_pos_y > graphics::size(ctx).1 {
            self.ballis_pos_y = 100.0;
        } else {
            self.ballis_pos_y = self.ballis_pos_y % 800.0 + 1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::draw(ctx, &self.ballis_img, (Point2::new(100.0, self.ballis_pos_y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

// Creating a gamestate depends on having an SDL context to load resources.
// Creating a context depends on loading a config file.
// Loading a config file depends on having FS (or we can just fake our way around it
// by creating an FS and then throwing it away; the costs are not huge.)
pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("spritebatch", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
