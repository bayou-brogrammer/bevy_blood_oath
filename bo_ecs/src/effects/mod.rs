use bevy_ecs::{event::Events, prelude::*};

use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;

use crate::prelude::*;

mod damage;
pub use damage::*;

lazy_static! {
    pub static ref EFFECT_QUEUE: Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

#[derive(Debug, Clone)]
pub enum EffectType {
    Damage { amount: i32 },
    EntityDeath,
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

///////////////////////////////////////////////////////////////////////////////
/// Events
//////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct AffectEntity {
    pub entity: Entity,
    pub effect: EffectSpawner,
}

#[derive(Debug)]
pub struct AffectTile {
    pub tile_idx: usize,
    pub effect: EffectSpawner,
}

//////////////////////////////////////////////////////////////////////////////

pub fn add_effect(creator: Option<Entity>, effect_type: EffectType, targets: Targets) {
    EFFECT_QUEUE.lock().push_back(EffectSpawner {
        creator,
        effect_type,
        targets,
    });
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    match effect {
        EffectType::Damage { .. } => true,
        _ => false,
    }
}

pub fn effects_queue(
    mut affect_entity_event: EventWriter<AffectEntity>,
    mut affect_tile_event: EventWriter<AffectTile>,
) {
    if EFFECT_QUEUE.lock().is_empty() {
        return;
    }

    for effect in EFFECT_QUEUE.lock().drain(..) {
        target_applicator(&effect, &mut affect_entity_event, &mut affect_tile_event);
    }
}

fn target_applicator(
    effect: &EffectSpawner,
    affect_entity_event: &mut EventWriter<AffectEntity>,
    affect_tile_event: &mut EventWriter<AffectTile>,
) {
    match &effect.targets {
        // Entity
        Targets::Single { target } => affect_entity_event.send(AffectEntity {
            entity: *target,
            effect: effect.clone(),
        }),
        Targets::TargetList { targets } => {
            let batch = targets
                .iter()
                .map(|entity| AffectEntity {
                    entity: *entity,
                    effect: effect.clone(),
                })
                .collect::<Vec<_>>();
            affect_entity_event.send_batch(batch.into_iter());
        }
        // Tile
        Targets::Tile { tile_idx } => affect_tile_event.send(AffectTile {
            tile_idx: *tile_idx,
            effect: effect.clone(),
        }),
        Targets::Tiles { tiles } => {
            let batch = tiles
                .iter()
                .map(|tile_idx| AffectTile {
                    tile_idx: *tile_idx,
                    effect: effect.clone(),
                })
                .collect::<Vec<_>>();

            affect_tile_event.send_batch(batch.into_iter());
        }
    };
}

pub fn affect_entity(
    mut affects: ResMut<Events<AffectEntity>>,
    mut damage_event: EventWriter<DamageEvent>,
    mut death_event: EventWriter<DeathEvent>,
) {
    for AffectEntity { effect, entity } in affects.drain() {
        match &effect.effect_type {
            EffectType::Damage { .. } => damage_event.send(DamageEvent {
                target: entity,
                effect,
            }),
            EffectType::EntityDeath => death_event.send(DeathEvent(entity)),
        }
    }
}

pub fn affect_tile(
    mut affects: ResMut<Events<AffectTile>>,
    mut affect_entity_event: EventWriter<AffectEntity>,
) {
    for AffectTile { tile_idx, effect } in affects.drain() {
        if tile_effect_hits_entities(&effect.effect_type) {
            let batch = bo_map::spatial::get_tile_content_clone(tile_idx)
                .iter()
                .map(|entity| AffectEntity {
                    entity: *entity,
                    effect: effect.clone(),
                })
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
