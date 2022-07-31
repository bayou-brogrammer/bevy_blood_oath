use super::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;

mod damage;
mod particle;
mod queries;
mod triggers;

pub use damage::*;
pub use particle::*;
pub use queries::*;
pub use triggers::*;

lazy_static! {
    pub static ref EFFECT_QUEUE: Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

#[derive(Debug)]
pub enum EffectType {
    Bloodstain(RGB),
    EntityDeath,
    Damage { amount: i32 },
    Healing { amount: i32 },
    Confusion { turns: i32 },
    ItemUse { item: Entity },
    Particle { glyph: FontCharType, color: ColorPair, lifespan: f32 },
}

impl EffectType {
    pub fn new_particle(glyph: FontCharType, color: ColorPair, lifespan: f32) -> Self {
        EffectType::Particle { glyph, color, lifespan }
    }

    pub fn new_healing(amount: i32) -> Self {
        EffectType::Healing { amount }
    }
}

#[derive(Clone, Debug)]
pub enum Targets {
    Single { target: Entity },
    TargetList { targets: Vec<Entity> },
    Tile { tile_idx: usize },
    Tiles { tiles: Vec<usize> },
}

#[derive(Debug)]
pub struct EffectSpawner {
    pub creator: Option<Entity>,
    pub effect_type: EffectType,
    pub targets: Targets,
}

pub fn add_effect(creator: Option<Entity>, effect_type: EffectType, targets: Targets) {
    EFFECT_QUEUE.lock().push_back(EffectSpawner { creator, effect_type, targets });
}

pub fn run_effects_queue(ecs: &mut World) {
    loop {
        let effect: Option<EffectSpawner> = EFFECT_QUEUE.lock().pop_front();
        if let Some(effect) = effect {
            target_applicator(ecs, &effect);
        } else {
            break;
        }
    }
}

fn target_applicator(ecs: &mut World, effect: &EffectSpawner) {
    if let EffectType::ItemUse { item } = effect.effect_type {
        triggers::item_trigger(ecs, effect.creator, item, &effect.targets);
    } else {
        match &effect.targets {
            Targets::Tile { tile_idx } => affect_tile(ecs, effect, *tile_idx),
            Targets::Tiles { tiles } => {
                tiles.iter().for_each(|tile_idx| affect_tile(ecs, effect, *tile_idx))
            }
            Targets::Single { target } => affect_entity(ecs, effect, *target),
            Targets::TargetList { targets } => {
                targets.iter().for_each(|entity| affect_entity(ecs, effect, *entity))
            }
        }
    }
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    matches!(
        effect,
        EffectType::Damage { .. } | EffectType::Healing { .. } | EffectType::Confusion { .. }
    )
}

fn affect_tile(ecs: &mut World, effect: &EffectSpawner, tile_idx: usize) {
    if tile_effect_hits_entities(&effect.effect_type) {
        let content = crate::spatial::get_tile_content_clone(tile_idx as usize);
        content.iter().for_each(|entity| affect_entity(ecs, effect, *entity));
    }

    match &effect.effect_type {
        EffectType::Bloodstain(color) => damage::bloodstain(ecs, tile_idx, color),
        EffectType::Particle { .. } => particle::particle_to_tile(ecs, tile_idx, effect),
        _ => {}
    }
}

fn affect_entity(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    match &effect.effect_type {
        EffectType::Damage { .. } => damage::inflict_damage(ecs, effect, target),
        EffectType::EntityDeath => damage::death(ecs, effect, target),
        EffectType::Healing { .. } => damage::heal_damage(ecs, effect, target),
        EffectType::Confusion { .. } => damage::add_confusion(ecs, effect, target),
        EffectType::Particle { .. } => {
            if let Some(pos) = entity_position(ecs, target) {
                particle::particle_to_tile(ecs, pos, effect)
            }
        }
        EffectType::Bloodstain(color) => {
            if let Some(pos) = entity_position(ecs, target) {
                damage::bloodstain(ecs, pos, color)
            }
        }
        _ => {}
    }
}
