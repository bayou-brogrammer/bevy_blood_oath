use crate::prelude::*;
use bo_utils::impl_new;
use bracket_terminal::prelude::{ColorPair, FontCharType};

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    EntityDeath,
    Damage { amount: i32 },
    ItemUse { item: Entity },
    Healing { amount: i32 },
    Particle { lifespan: f32, color: ColorPair, glyph: FontCharType },
}

#[derive(Debug, Clone)]
pub enum Targets {
    Single { target: Entity },
    Tile { tile_idx: usize },
    Tiles { tiles: Vec<usize> },
    TargetList { targets: Vec<Entity> },
}

#[derive(Clone, Debug)]
pub struct EffectSpawner {
    pub targets: Targets,
    pub effect_type: EffectType,
    pub creator: Option<Entity>,
}

impl_new!(EffectSpawner, creator: Option<Entity>, targets: Targets, effect_type: EffectType);
