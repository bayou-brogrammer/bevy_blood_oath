use crate::prelude::*;

#[derive(Bundle, Component, Debug)]
pub struct RenderBundle {
    pub glyph: Glyph,
    pub position: Position,
}

#[derive(Bundle, Component, Debug)]
pub struct EntityBundle<TAG: Component> {
    pub tag: TAG,
    pub name: Name,
    pub fov: FieldOfView,
    pub stats: CombatStats,
    pub description: Description,
}

#[derive(Bundle, Component, Debug)]
pub struct MonsterBundle {
    #[bundle]
    pub monster: EntityBundle<Monster>,
    pub blocks: BlocksTile,
}

#[derive(Bundle)]
pub struct DeadBundle {
    pub name: Name,
    pub glyph: Glyph,
}
