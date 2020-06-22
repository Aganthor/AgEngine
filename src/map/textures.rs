use ggez;

use ggez::{filesystem, Context, graphics};
use std::collections::HashMap;
use super::tiles;

pub struct TextureLoader {
    textures: HashMap<tiles::TileType, graphics::Image>
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        let texture_map: HashMap<tiles::TileType, graphics::Image> = HashMap::new();
        TextureLoader {
            textures: texture_map
        }
    }

    pub fn load_textures(mut self, ctx: &mut Context) {
         //let mut texture_file = graphics::Image::new(ctx, "/grass.png").unwrap();       
         //self.textures.insert(tiles::TileType::Grass, texture_file);
         let dir_content: Vec<_> = filesystem::read_dir(ctx, "/").unwrap().collect();
         for item in dir_content {
             let mut filename = item.into_os_string().into_string().unwrap();
             filename.retain(|c| c != '/');
             println!("{}", filename);
         }
    }
}