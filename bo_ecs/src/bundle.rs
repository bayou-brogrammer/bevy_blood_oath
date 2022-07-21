use crate::prelude::*;

use bevy::prelude::Bundle;
use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::Point;
use bracket_terminal::FontCharType;

#[derive(Bundle, Component)]
pub struct RenderBundle {
    pub glyph: Glyph,
    pub position: Position,
}

impl RenderBundle {
    pub fn new(glyph: FontCharType, color: ColorPair, order: RenderOrder, pt: Point) -> Self {
        Self {
            glyph: Glyph::new(glyph, color, order),
            position: Position::new(pt),
        }
    }
}

#[derive(Bundle, Component)]
pub struct EntityBundle<TAG: Component> {
    pub tag: TAG,
    pub name: Naming,
    pub description: Description,
}

impl<TAG: Component> EntityBundle<TAG> {
    pub fn new(tag: TAG, name: &str, description: &str) -> Self {
        Self {
            tag,
            name: Naming(name.to_string()),
            description: Description(description.to_string()),
        }
    }
}

#[derive(Bundle, Component)]
pub struct FighterBundle<TYPE: Component> {
    #[bundle]
    pub entity: EntityBundle<TYPE>,
    pub fov: FieldOfView,
    pub stats: CombatStats,
}

impl<TYPE: Component> FighterBundle<TYPE> {
    pub fn new(entity: EntityBundle<TYPE>, fov: FieldOfView, stats: CombatStats) -> Self {
        Self { entity, fov, stats }
    }
}

#[derive(Bundle, Component)]
pub struct MonsterBundle {
    #[bundle]
    pub fighter: FighterBundle<Monster>,
    pub blocks: BlocksTile,
}

impl MonsterBundle {
    pub fn new(fighter: FighterBundle<Monster>) -> Self {
        Self {
            fighter,
            blocks: BlocksTile,
        }
    }
}

#[derive(Bundle, Component)]
pub struct ItemBundle<ITEM: Component> {
    #[bundle]
    pub entity: EntityBundle<Item>,
    #[bundle]
    pub render: RenderBundle,
    pub item: ITEM,
}

impl<ITEM: Component> ItemBundle<ITEM> {
    pub fn new(entity: EntityBundle<Item>, render: RenderBundle, item: ITEM) -> Self {
        Self {
            entity,
            render,
            item,
        }
    }
}
