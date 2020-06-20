
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::GameResult;

use map;
use tiles;

pub struct Level {
    level_data: Vec<TileInfo>,
}

impl Level {
    pub fn new() -> Level {
        Level {

        }
    }
}

impl Drawable for Level {
    fn draw(&self, ctx: Context, param: DrawParam) -> GameResult {
        Ok(())
    }
}

