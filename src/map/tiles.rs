pub const TILE_SIZE: u32 = 32;
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

