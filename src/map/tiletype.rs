use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Road,
    Door,
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct GameTile {
    pub cost: f32,
    pub opaque: bool,
    pub walkable: bool,
    pub color: ColorPair,
    pub tile_type: TileType,
    pub glyph: FontCharType,
}

impl PartialEq for GameTile {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
            && self.opaque == other.opaque
            && self.walkable == other.walkable
            && self.color == other.color
            && self.tile_type == other.tile_type
            && self.glyph == other.glyph
    }
}
impl Eq for GameTile {}

impl std::hash::Hash for GameTile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.opaque.hash(state);
        self.walkable.hash(state);
        self.tile_type.hash(state);
        self.glyph.hash(state);
    }
}

impl_new!(
    GameTile,
    cost: f32,
    opaque: bool,
    walkable: bool,
    color: ColorPair,
    tile_type: TileType,
    glyph: FontCharType
);

impl Default for GameTile {
    fn default() -> Self {
        Self {
            cost: 1.0,
            opaque: false,
            walkable: true,
            tile_type: TileType::Wall,
            glyph: FontCharType::default(),
            color: ColorPair::new((0, 0, 0), (0, 0, 0)),
        }
    }
}

impl GameTile {
    pub fn door() -> Self {
        Self {
            opaque: true,
            walkable: false,
            glyph: to_cp437('+'),
            tile_type: TileType::Door,
            color: ColorPair::new(CHOCOLATE, BLACK),
            ..Default::default()
        }
    }

    pub fn floor() -> Self {
        Self {
            glyph: to_cp437('.'),
            tile_type: TileType::Floor,
            color: ColorPair::new(DARK_GRAY, BLACK),
            ..Default::default()
        }
    }

    pub fn wall() -> Self {
        Self {
            opaque: true,
            walkable: false,
            glyph: to_cp437('#'),
            tile_type: TileType::Wall,
            color: ColorPair::new(DARK_GRAY, BLACK),
            ..Default::default()
        }
    }

    pub fn stairs_down() -> Self {
        Self {
            glyph: to_cp437('>'),
            tile_type: TileType::DownStairs,
            color: ColorPair::new(CYAN, BLACK),
            ..Default::default()
        }
    }

    pub fn stairs_up() -> Self {
        Self {
            glyph: to_cp437('<'),
            tile_type: TileType::UpStairs,
            color: ColorPair::new(CYAN, BLACK),
            ..Default::default()
        }
    }

    pub fn road() -> Self {
        Self {
            cost: 0.8,
            glyph: to_cp437('≡'),
            tile_type: TileType::Road,
            color: ColorPair::new(GRAY, BLACK),
            ..Default::default()
        }
    }

    pub fn grass() -> Self {
        Self {
            cost: 1.1,
            glyph: to_cp437('"'),
            tile_type: TileType::Grass,
            color: ColorPair::new(GREEN, BLACK),
            ..Default::default()
        }
    }

    pub fn bridge() -> Self {
        Self {
            glyph: to_cp437('.'),
            tile_type: TileType::Bridge,
            color: ColorPair::new(CHOCOLATE, BLACK),
            ..Default::default()
        }
    }

    pub fn gravel() -> Self {
        Self {
            glyph: to_cp437(';'),
            tile_type: TileType::Gravel,
            color: ColorPair::new(GRAY44, BLACK),
            ..Default::default()
        }
    }

    pub fn wood_floor() -> Self {
        Self {
            glyph: to_cp437('░'),
            tile_type: TileType::WoodFloor,
            color: ColorPair::new(CHOCOLATE, BLACK),
            ..Default::default()
        }
    }

    pub fn stalagmite() -> Self {
        Self {
            opaque: true,
            walkable: false,
            glyph: to_cp437('╥'),
            tile_type: TileType::Stalagmite,
            color: ColorPair::new(GRAY44, BLACK),
            ..Default::default()
        }
    }

    pub fn stalactite() -> Self {
        Self {
            opaque: true,
            walkable: false,
            glyph: to_cp437('╨'),
            tile_type: TileType::Stalactite,
            color: ColorPair::new(GRAY44, BLACK),
            ..Default::default()
        }
    }

    pub fn deep_water() -> Self {
        Self {
            walkable: false,
            glyph: to_cp437('~'),
            tile_type: TileType::DeepWater,
            color: ColorPair::new(BLUE, BLACK),
            ..Default::default()
        }
    }

    pub fn shallow_water() -> Self {
        Self {
            cost: 1.2,
            glyph: to_cp437('~'),
            tile_type: TileType::ShallowWater,
            color: ColorPair::new(CYAN, BLACK),
            ..Default::default()
        }
    }
}
