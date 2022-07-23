use bracket_terminal::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Capsule,
    Wall,
    Floor,
    Outside,
    StairsDown,
    StairsUp,
}

#[derive(Clone)]
pub struct GameTile {
    pub glyph: FontCharType,
    pub color: ColorPair,
    pub opaque: bool,
    pub walkable: bool,
    pub tile_type: TileType,
}

impl GameTile {
    pub fn default() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('.'),
            tile_type: TileType::Floor,
            color: ColorPair::new(GREEN, BLACK),
        }
    }

    pub fn empty() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437(' '),
            tile_type: TileType::Empty,
            color: ColorPair::new(DARK_GRAY, BLACK),
        }
    }

    pub fn floor() -> Self {
        Self {
            opaque: false,
            walkable: true,
            glyph: to_cp437('.'),
            tile_type: TileType::Floor,
            color: ColorPair::new(WHITE, BLACK),
        }
    }

    pub fn wall() -> Self {
        Self {
            opaque: true,
            walkable: false,
            glyph: to_cp437('#'),
            tile_type: TileType::Wall,
            color: ColorPair::new(WHITE, BLACK),
        }
    }

    // pub fn window() -> Self {
    //     Self {
    //         glyph: to_cp437('#'),
    //         color: ColorPair::new(DARK_CYAN, BLACK),
    //         blocked: true,
    //         opaque: false,
    //         tile_type: TileType::Wall,
    //         contents: Vec::new(),
    //     }
    // }

    pub fn stairs_down() -> Self {
        Self {
            opaque: false,

            walkable: true,
            glyph: to_cp437('>'),
            tile_type: TileType::Wall,
            color: ColorPair::new(WHITE, BLACK),
        }
    }

    // pub fn stairs_up() -> Self {
    //     Self {
    //         glyph: to_cp437('<'),
    //         color: ColorPair::new(WHITE, BLACK),
    //         blocked: false,
    //         opaque: false,
    //         tile_type: TileType::StairsUp,
    //         contents: Vec::new(),
    //     }
    // }
}
