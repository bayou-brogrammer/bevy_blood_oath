use bracket_terminal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TileType {
    Wall,
    Stalactite,
    Stalagmite,
    Floor,
    DownStairs,
    Road,
    Grass,
    ShallowWater,
    DeepWater,
    WoodFloor,
    Bridge,
    Gravel,
    UpStairs,
}

pub fn tile_walkable(tt: TileType) -> bool {
    matches!(
        tt,
        TileType::Floor
            | TileType::DownStairs
            | TileType::Road
            | TileType::Grass
            | TileType::ShallowWater
            | TileType::WoodFloor
            | TileType::Bridge
            | TileType::Gravel
            | TileType::UpStairs
    )
}

pub fn tile_opaque(tt: TileType) -> bool {
    matches!(tt, TileType::Wall | TileType::Stalactite | TileType::Stalagmite)
}

pub fn tile_cost(tt: TileType) -> f32 {
    match tt {
        TileType::Road => 0.8,
        TileType::Grass => 1.1,
        TileType::ShallowWater => 1.2,
        _ => 1.0,
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameTile {
    pub opaque: bool,
    pub walkable: bool,
    pub color: ColorPair,
    pub tile_type: TileType,
    pub glyph: FontCharType,
}

impl GameTile {
    pub fn floor() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('.'),
            tile_type: TileType::Floor,
            color: ColorPair::new(DARK_GRAY, BLACK),
        }
    }

    pub fn wall() -> Self {
        Self {
            opaque: true,
            walkable: false,
            glyph: to_cp437('#'),
            tile_type: TileType::Wall,
            color: ColorPair::new(DARK_GRAY, BLACK),
        }
    }

    pub fn stairs_down() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('>'),
            tile_type: TileType::DownStairs,
            color: ColorPair::new(WHITE, BLACK),
        }
    }

    pub fn stairs_up() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('<'),
            tile_type: TileType::UpStairs,
            color: ColorPair::new(WHITE, BLACK),
        }
    }
}
