use crate::prelude::*;

use bo_utils::impl_new;
use bracket_geometry::prelude::Point;
use bracket_terminal::prelude::{ColorPair, FontCharType};

#[derive(Bundle, Component)]
pub struct RenderBundle {
    pub glyph: Glyph,
    pub position: Position,
}

impl RenderBundle {
    pub fn new(glyph: FontCharType, color: ColorPair, order: RenderOrder, pt: Point) -> Self {
        Self { glyph: Glyph::new(glyph, color, order), position: Position::new(pt) }
    }
}

#[derive(Bundle, Component)]
pub struct EntityBundle<TAG: Component> {
    pub tag: TAG,
    pub name: Naming,
}

impl<TAG: Component> EntityBundle<TAG> {
    pub fn new(tag: TAG, name: &str) -> Self {
        Self { tag, name: Naming(name.to_string()) }
    }
}

#[derive(Bundle, Component)]
pub struct FighterBundle {
    pub fov: FieldOfView,
    pub stats: CombatStats,
}

impl FighterBundle {
    pub fn new(fov: FieldOfView, stats: CombatStats) -> Self {
        Self { fov, stats }
    }
}

#[derive(Bundle, Component)]
pub struct PlayerBundle {
    pub tag: Player,
    #[bundle]
    pub fighter: FighterBundle,
}

impl PlayerBundle {
    pub fn new(fighter: FighterBundle) -> Self {
        Self { fighter, tag: Player }
    }
}

#[derive(Bundle, Component)]
pub struct MonsterBundle {
    pub tag: Monster,
    #[bundle]
    pub fighter: FighterBundle,
    pub blocks: BlocksTile,
}

impl MonsterBundle {
    pub fn new(fighter: FighterBundle) -> Self {
        Self { fighter, blocks: BlocksTile, tag: Monster }
    }
}

#[derive(Bundle, Component)]
pub struct ItemBundle {
    #[bundle]
    pub entity: EntityBundle<Item>,
    #[bundle]
    pub render: RenderBundle,
}

impl_new!(ItemBundle, entity: EntityBundle<Item>, render: RenderBundle);

#[derive(Bundle, Component)]
pub struct ParticleBundle {
    glyph: Glyph,
    position: Position,
    lifetime: ParticleLifetime,
}

impl_new!(ParticleBundle, position: Position, glyph: Glyph, lifetime: ParticleLifetime);
