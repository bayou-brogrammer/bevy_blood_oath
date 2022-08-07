use super::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;

mod damage;
mod hunger;
mod particle;
mod queries;
mod triggers;

pub use damage::*;
pub use hunger::*;
pub use particle::*;
pub use queries::*;
pub use triggers::*;

lazy_static! {
    pub static ref EFFECT_QUEUE: Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

#[derive(Debug)]
pub enum EffectType {
    WellFed,
    EntityDeath,
    Bloodstain(RGB),
    Damage(i32),
    Healing(i32),
    Confusion(i32),
    ItemUse(Entity),
    TriggerFire(Entity),
    Particle { glyph: FontCharType, color: ColorPair, lifespan: f32 },
}

impl EffectType {
    pub fn new_particle(glyph: FontCharType, color: ColorPair, lifespan: f32) -> Self {
        EffectType::Particle { glyph, color, lifespan }
    }

    pub fn new_healing(amount: i32) -> Self { EffectType::Healing(amount) }
}

#[derive(Clone, Debug)]
pub enum Targets {
    Tile(usize),
    Single(Entity),
    Tiles(Vec<usize>),
    TargetList(Vec<Entity>),
}

#[derive(Debug)]
pub struct EffectSpawner {
    pub targets: Targets,
    pub creator: Option<Entity>,
    pub effect_type: EffectType,
}

///////////////////////////////////////////////////////////////////////////////////////////////
// Add Effects
///////////////////////////////////////////////////////////////////////////////////////////////

pub fn add_effect(creator: Option<Entity>, effect_type: EffectType, targets: Targets) {
    EFFECT_QUEUE.lock().push_back(EffectSpawner { creator, effect_type, targets });
}

pub fn add_single_damage_effect(creator: Option<Entity>, target: Entity, amount: i32) {
    add_effect(creator, EffectType::Damage(amount), Targets::Single(target));
}

pub fn add_single_healing_effect(creator: Option<Entity>, target: Entity, amount: i32) {
    add_effect(creator, EffectType::Healing(amount), Targets::Single(target));
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub fn run_effects_queue(world: &mut World) {
    loop {
        let effect: Option<EffectSpawner> = EFFECT_QUEUE.lock().pop_front();
        if let Some(effect) = effect {
            target_applicator(world, &effect);
        } else {
            break;
        }
    }
}

fn target_applicator(world: &mut World, effect: &EffectSpawner) {
    if let EffectType::ItemUse(item) = effect.effect_type {
        triggers::item_trigger(world, effect.creator, item, &effect.targets);
    } else if let EffectType::TriggerFire(trigger) = effect.effect_type {
        triggers::trigger(world, effect.creator, trigger, &effect.targets);
    } else {
        match &effect.targets {
            Targets::Single(target) => affect_entity(world, effect, *target),
            Targets::Tile(tile_idx) => affect_tile(world, effect, *tile_idx),
            Targets::Tiles(tiles) => tiles.iter().for_each(|tile_idx| affect_tile(world, effect, *tile_idx)),
            Targets::TargetList(targets) => {
                targets.iter().for_each(|entity| affect_entity(world, effect, *entity))
            }
        }
    }
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    matches!(
        effect,
        EffectType::Damage { .. }
            | EffectType::Healing { .. }
            | EffectType::Confusion { .. }
            | EffectType::WellFed
    )
}

fn affect_tile(world: &mut World, effect: &EffectSpawner, tile_idx: usize) {
    if tile_effect_hits_entities(&effect.effect_type) {
        crate::spatial::for_each_tile_content(tile_idx, |entity| affect_entity(world, effect, entity));
    }

    match &effect.effect_type {
        EffectType::Bloodstain(color) => damage::bloodstain(world, tile_idx, color),
        EffectType::Particle { .. } => particle::particle_to_tile(world, tile_idx, effect),
        _ => {}
    }
}

fn affect_entity(world: &mut World, effect: &EffectSpawner, target: Entity) {
    match &effect.effect_type {
        EffectType::WellFed => hunger::well_fed(world, effect, target),
        EffectType::EntityDeath => damage::death(world, effect, target),
        EffectType::Healing { .. } => damage::heal_damage(world, effect, target),
        EffectType::Damage { .. } => damage::inflict_damage(world, effect, target),
        EffectType::Confusion { .. } => damage::add_confusion(world, effect, target),
        EffectType::Particle { .. } => {
            if let Some(pos) = entity_position(world, target) {
                particle::particle_to_tile(world, pos, effect)
            }
        }
        EffectType::Bloodstain(color) => {
            if let Some(pos) = entity_position(world, target) {
                damage::bloodstain(world, pos, color)
            }
        }
        _ => {}
    }
}
