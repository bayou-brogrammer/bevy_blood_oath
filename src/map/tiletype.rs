use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct GameTile {
    pub opaque: bool,
    pub walkable: bool,
    pub color: ColorPair,
    pub tile_type: TileType,
    pub glyph: FontCharType,
}

impl GameTile {
    pub fn set_as_floor(&mut self) {
        *self = GameTile::floor()
    }

    pub fn set_as_wall(&mut self) {
        *self = GameTile::wall()
    }
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
            color: ColorPair::new(CYAN, BLACK),
        }
    }

    pub fn stairs_up() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('<'),
            tile_type: TileType::UpStairs,
            color: ColorPair::new(CYAN, BLACK),
        }
    }

    pub fn deep_water() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('~'),
            tile_type: TileType::DeepWater,
            color: ColorPair::new(BLUE, BLACK),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Road,
    Grass,
    Bridge,
    Gravel,
    UpStairs,
    DeepWater,
    WoodFloor,
    Stalactite,
    Stalagmite,
    DownStairs,
    ShallowWater,
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
