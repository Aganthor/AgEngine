use ggez;
use ggez::event::{self, EventHandler};
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::env;
use std::path;

mod map;
use map::map::{Map, MapBuilder};

struct MyGame {
    world_map: Map
}

impl MyGame {
    fn new(ctx: &mut Context) -> MyGame {
        let mut map = MapBuilder::new()
            .with_seed(192384)
            .with_frequency(0.03)
            .with_gain(2.5)
            .with_lacunarity(0.55)
            .with_octaves(2)
            .with_size(20)
            .build();

        map.generate_noise_map();
        map.prepare_textures(ctx);
        map.generate_level();

        MyGame {
            world_map: map
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        graphics::draw(ctx, &self.world_map, DrawParam::default()).unwrap();

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
