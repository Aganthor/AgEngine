use ggez;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer;
use ggez::{conf, graphics, Context, ContextBuilder, GameResult};

use std::env;
use std::path;

use rand::Rng;

mod map;
use map::map::{Map, MapBuilder};
use map::tiles;


//const MAX_ZOOM_IN: f32 = 1.0;
//const MAX_ZOOM_OUT: f32 = 0.3;

struct MyGame {
    world_map: Map,
    keysdown : Vec<KeyCode>,
    origin: Point2<f32>,
    zoom: f32,
}

impl MyGame {
    fn new(ctx: &mut Context) -> MyGame {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        println!("Map seed is {}.", seed);
        let mut map = MapBuilder::new()
            .with_seed(seed)
            .with_frequency(0.03)
            .with_gain(2.5)
            .with_lacunarity(0.55)
            .with_octaves(2)
            .with_size(100)
            .build();

        map.generate_noise_map();
        map.prepare_textures(ctx);
        map.generate_level();

        MyGame {
            world_map: map,
            keysdown: Vec::new(),
            origin: Point2::new(0.0, 0.0),
            zoom: 1.0
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        const SCROLL_SIZE: f32 = 32.0;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            for keycode in &self.keysdown {
                if keycode == &KeyCode::Up {
                    self.origin.y -= SCROLL_SIZE;
                }
                if keycode == &KeyCode::Down {
                    self.origin.y += SCROLL_SIZE;
                }
                if keycode == &KeyCode::Left {
                    self.origin.x -= SCROLL_SIZE;
                }
                if keycode == &KeyCode::Right {
                    self.origin.x += SCROLL_SIZE;
                }
            }
            let window = graphics::window(ctx);
            let window_size = window.get_inner_size().unwrap();

            if self.origin.x < 0.0 {
                self.origin.x = 0.0;
            } else if self.origin.x > self.world_map.map_max_size() - window_size.width as f32 {
                self.origin.x = self.world_map.map_max_size() - window_size.width as f32 - SCROLL_SIZE;
            }
            if self.origin.y < 0.0 {
                self.origin.y = 0.0;
            } else if self.origin.y > self.world_map.map_max_size() - window_size.height as f32 {
                self.origin.y = self.world_map.map_max_size() - window_size.height as f32 - SCROLL_SIZE;
            }            
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymode: KeyMods, _repeat: bool) {
        self.keysdown.push(keycode);
        self.keysdown.dedup_by_key(|x| *x);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymode: KeyMods) {
        self.keysdown.retain(|&x| x != keycode);
    }    

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        if y > 0.0 {
            self.zoom -= 0.1;
// //            if self.zoom > MAX_ZOOM_IN {
// //                self.zoom = MAX_ZOOM_IN;
// //            }
        } else if y < 0.0 {
            self.zoom += 0.1;
// //            if self.zoom < MAX_ZOOM_OUT {
// //                self.zoom = MAX_ZOOM_OUT;
// //            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        graphics::draw(ctx, &self.world_map, 
            DrawParam::default()
            .offset(Point2::new(self.origin.x, self.origin.y))
            .scale(Vector2::new(self.zoom, self.zoom))
        ).unwrap();

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
        .window_setup(conf::WindowSetup::default().title("AgEngine"))
        .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0))
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
