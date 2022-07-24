use crate::prelude::*;
use bo_utils::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;

mod damage;
mod particle;
mod triggers;

pub use damage::*;
pub use particle::*;
pub use triggers::*;

lazy_static! {
    pub static ref EFFECT_QUEUE: Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    EntityDeath,
    Damage { amount: i32 },
    Healing { amount: i32 },
    Confusion { turns: i32 },
    ItemUse { item: Entity },
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

//////////////////////////////////////////////////////////////////////////////

pub fn run_effects_queue(ecs: &mut World) {
    loop {
        let effect: Option<EffectSpawner> = EFFECT_QUEUE.lock().pop_front();
        if let Some(effect) = effect {
            if let EffectType::ItemUse { item } = effect.effect_type {
                triggers::item_trigger(ecs, effect.creator, item, &effect.targets);
            } else {
                match &effect.targets {
                    Targets::Tile { tile_idx } => affect_tile(ecs, &effect, *tile_idx),
                    Targets::Single { target } => affect_entity(ecs, &effect, *target),
                    Targets::Tiles { tiles } => {
                        tiles.iter().for_each(|tile_idx| affect_tile(ecs, &effect, *tile_idx))
                    }
                    Targets::TargetList { targets } => {
                        targets.iter().for_each(|entity| affect_entity(ecs, &effect, *entity))
                    }
                }
            }
        } else {
            break;
        }
    }
}

pub fn add_effect(creator: Option<Entity>, effect_type: EffectType, targets: Targets) {
    EFFECT_QUEUE.lock().push_back(EffectSpawner { creator, effect_type, targets });
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    match effect {
        EffectType::Damage { .. } => true,
        EffectType::Healing { .. } => true,
        EffectType::Confusion { .. } => true,
        _ => false,
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
                particle_to_tile(ecs, pos, &effect)
            }
        }
        _ => {}
    }
}
fn affect_tile(ecs: &mut World, effect: &EffectSpawner, tile_idx: usize) {
    if tile_effect_hits_entities(&effect.effect_type) {
        let content = crate::spatial::get_tile_content_clone(tile_idx as usize);
        content.iter().for_each(|entity| affect_entity(ecs, effect, *entity));
    }

    match &effect.effect_type {
        EffectType::Particle { .. } => particle_to_tile(ecs, tile_idx, &effect),
        _ => {}
    }
}
