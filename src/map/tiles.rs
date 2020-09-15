pub const TILE_SIZE: u32 = 32;

#[derive(Hash, PartialEq, Eq)]
pub enum TileType {
    DeepWater,
    Dirt,
    Grass,
    Forest,
    Rock,
    Sand,
    Savannah,
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
            "forest.png" => TileType::Forest,
            "rock.png" => TileType::Rock,
            "sand.png" => TileType::Sand,
            "savannah.png" => TileType::Savannah,
            "shallow_water.png" => TileType::ShallowWater,
            "shore.png" => TileType::Shore,
            "snow.png" => TileType::Snow,
            _ => TileType::None,
        }
    }
}

pub struct TileInfo {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
    pub explored: bool,
    pub block_view: bool,
    pub walkable: bool,
}

impl TileInfo {
    pub fn new(x: usize, y: usize, tile_type: TileType, is_walkable: bool) -> TileInfo {
        TileInfo {
            x: x,
            y: y,
            tile_type: tile_type,
            explored: false,
            block_view: false,
            walkable: is_walkable,
        }
    }
}
