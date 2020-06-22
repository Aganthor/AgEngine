pub const TILE_SIZE: u32 = 32;

#[derive(Hash, PartialEq, Eq)]
pub enum TileType {
    DeepWater,
    Dirt,
    Grass,
    Rock,
    Sand,
    ShallowWater,
    Shore,
    Snow,
    None,
}

impl From<&str> for TileType {
    fn from(tile_type: &str) -> Self {
        match tile_type {
            "deep_water.png" => TileType::DeepWater,
            "dirt.png" => TileType::Dirt,
            "grass.png" => TileType::Grass,
            "rock.png" => TileType::Rock,
            "sand.png" => TileType::Sand,
            "shallow_water.png" => TileType::ShallowWater,
            "shore.png" => TileType::Shore,
            "snow.png" => TileType::Snow,
            _ => TileType::None
        }
    }
}

pub struct TileInfo {
    x: i32,
    y: i32,
    tile_type: TileType,
    explored: bool,
    block_view: bool,
    walkable: bool
}

impl TileInfo {
    pub fn new(x: i32, y: i32) -> TileInfo {
        TileInfo {
            x: x,
            y: y,
            tile_type: TileType::None,
            explored: false,
            block_view: false,
            walkable: false
        }
    }
}

