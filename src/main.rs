use ggez;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;

use std::env;
use std::path;

mod map;
use map::map::Map;

struct MyGame {

}

impl MyGame {
    fn new(ctx: &mut Context) -> MyGame {
        MyGame {
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        graphics::present(ctx)
    }
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MAINFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets/tiles");
        path
    } else {
        path::PathBuf::from("./assets/tiles")
    };
    
    let (mut ctx, mut event_loop) = ContextBuilder::new("AgEngine", "Luc Bergeron")
        .add_resource_path(resource_dir)
        .build()
        .expect("Could not create ggez context...");

    let mut my_game = MyGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly!"),
        Err(e) => println!("Error occured: {}", e),
    }
}

//TODO
// Integrate ggez to render the noise in a live fashion.
// Integrate x to have widget to control the noise settings.
