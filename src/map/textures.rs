use ggez;

use ggez::{filesystem, ContextBuilder, GameResult, Context};
use std::collections::HashMap;
//use tiles;

pub struct TextureLoader {
    //textures: HashMap
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            //textures: HashMap::new()
        }
    }

    pub fn load_textures(mut self, ctx: &mut Context) {
         //let mut texture_file = graphics::Image::new(ctx, "/grass.png").unwrap();       
         //self.textures.insert(tiles::TileType::Grass, texture_file);
         let dir_content: Vec<_> = filesystem::read_dir(ctx, "/").unwrap().collect();
         for item in dir_content {
             let filename    = item.into_os_string();
         }
    }
}