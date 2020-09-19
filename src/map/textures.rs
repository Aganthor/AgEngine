use ggez;

use super::tiles;
use ggez::{filesystem, graphics, Context};
use std::collections::HashMap;

pub struct TextureLoader {
    pub textures: HashMap<tiles::TileType, graphics::Image>,
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        let texture_map: HashMap<tiles::TileType, graphics::Image> = HashMap::new();
        TextureLoader {
            textures: texture_map,
        }
    }

    pub fn load_textures(&mut self, ctx: &mut Context) {
        let dir_content: Vec<_> = filesystem::read_dir(ctx, "/tiles").unwrap().collect();
        for item in dir_content {
            let filename = item.into_os_string().into_string().unwrap();
            let texture = graphics::Image::new(ctx, filename.to_string().clone()).unwrap();
            self.textures.insert(tiles::TileType::from(&filename[7..]), texture);
        }
    }
}
