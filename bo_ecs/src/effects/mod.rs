use crate::prelude::*;
use bo_map::prelude::Map;
use bo_utils::impl_new;

use bevy_ecs::{event::Events, prelude::*};
use bracket_algorithm_traits::prelude::Algorithm2D;
use bracket_terminal::prelude::*;
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

#[derive(Debug, Clone)]
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

//////////////////////////////////////////////////////////////////////////////

pub fn effects_queue(
    mut affect_entity_event: EventWriter<AffectEntity>,
    mut affect_tile_event: EventWriter<AffectTile>,
    mut item_trigger_event: EventWriter<ItemTrigger>,
) {
    if EFFECT_QUEUE.lock().is_empty() {
        return;
    }

    for effect in EFFECT_QUEUE.lock().drain(..) {
        if let EffectType::ItemUse { item } = effect.effect_type {
            item_trigger_event.send(ItemTrigger::new(item, effect.creator, effect.targets));
        } else {
            match &effect.targets {
                // Entity
                Targets::Single { target } => affect_entity_event.send(AffectEntity::new(*target, effect)),
                Targets::TargetList { targets } => {
                    let batch = targets
                        .iter()
                        .map(|entity| AffectEntity::new(*entity, effect.clone()))
                        .collect::<Vec<_>>();

                    affect_entity_event.send_batch(batch.into_iter());
                }
                // Tile
                Targets::Tile { tile_idx } => affect_tile_event.send(AffectTile::new(*tile_idx, effect)),
                Targets::Tiles { tiles } => {
                    let batch = tiles
                        .iter()
                        .map(|tile_idx| AffectTile::new(*tile_idx, effect.clone()))
                        .collect::<Vec<_>>();

                    affect_tile_event.send_batch(batch.into_iter());
                }
            };
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
        _ => false,
    }
}

pub fn affect_entity(
    mut affects: ResMut<Events<AffectEntity>>,
    mut damage_event: EventWriter<DamageEvent>,
    mut death_event: EventWriter<DeathEvent>,
    mut heal_event: EventWriter<HealEvent>,
    mut particle_event: EventWriter<ParticleEvent>,
    positions: Query<&Position>,
    map: Res<Map>,
) {
    for AffectEntity { effect, entity } in affects.drain() {
        match &effect.effect_type {
            EffectType::Damage { .. } => damage_event.send(DamageEvent::new(entity, effect)),
            EffectType::EntityDeath => death_event.send(DeathEvent(entity)),
            EffectType::Healing { .. } => heal_event.send(HealEvent::new(entity, effect)),
            EffectType::Particle { .. } => {
                if let Some(pos) = entity_position(entity, &positions) {
                    particle_event.send(ParticleEvent::new(map.point2d_to_index(pos), effect));
                }
            }
            _ => {}
        }
    }
}

pub fn affect_tile(mut affects: ResMut<Events<AffectTile>>, mut affect_entity_event: EventWriter<AffectEntity>) {
    for AffectTile { tile_idx, effect } in affects.drain() {
        if tile_effect_hits_entities(&effect.effect_type) {
            let batch = bo_map::spatial::get_tile_content_clone(tile_idx)
                .iter()
                .map(|entity| AffectEntity::new(*entity, effect.clone()))
                .collect::<Vec<_>>();

            affect_entity_event.send_batch(batch.into_iter());
        }
    }

    // match &effect.effect_type {
    //     EffectType::Bloodstain => damage::bloodstain(ecs, tile_idx),
    //     EffectType::Particle { .. } => particles::particle_to_tile(ecs, tile_idx, &effect),
    //     _ => {}
    // }
}
