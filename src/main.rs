use ggez;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;

use std::env;
use std::path;

struct MyGame {

}

impl MyGame {
    fn new(ctx: &mut Context) -> MyGame {
//        let grass_file = graphics::Image::new(ctx, "/grass.png").unwrap();

        MyGame {
//            grass: grass_file,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

/*        for x in 1..10 {
            let my_dest = ggez::nalgebra::Point2::new(x as f32 * 32.0, 20.0);
            graphics::draw(ctx, &self.grass, DrawParam::default().dest(my_dest))?;
        }*/

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
/*
    let textures_path = fs::read_dir("./assets/tiles").unwrap();
    for file in textures_path {
        println!("{}", file.unwrap().path().display());
    }
*/        
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
