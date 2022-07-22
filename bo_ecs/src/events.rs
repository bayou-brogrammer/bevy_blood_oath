use crate::prelude::*;
use bo_utils::impl_new;
use bracket_geometry::prelude::Point;

// Differently from the source project, which uses resources, we use Bevy's messaging system for move
// and attack messages.
// In the context of this project, it's a bit more ergonomic, but in larger ones, there advantages are
// more significant.
// Watch out! Events persist for two frames, which in this design is not a problem, but it's something
// important to know.

#[derive(Debug)]
pub struct WantsToMove {
    pub entity: Entity,
    // Event type fields don't need to be components; in this case we don't need to use PointC, but
    // it can be trivially done.
    pub destination: Point,
}

#[derive(Debug)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Debug)]
pub struct WantsToPickupItem {
    pub item: Entity,
    pub collected_by: Entity,
}

#[derive(Debug)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
    pub creator: Entity,
}

#[derive(Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
    pub dropper: Entity,
}

// pub struct ParticleRequestEvent {}

//////////////////////////////////////////////////////////////////////////////////////////////
/// Effects
//////////////////////////////////////////////////////////////////////////////////////////////

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

impl_new!(AffectEntity, entity: Entity, effect: EffectSpawner);
impl_new!(AffectTile, tile_idx: usize, effect: EffectSpawner);

/////////////
/// Triggers
/////////////

#[derive(Debug)]
pub struct ItemTrigger {
    pub item: Entity,
    pub targets: Targets,
    pub creator: Option<Entity>,
}

impl_new!(ItemTrigger, item: Entity, creator: Option<Entity>, targets: Targets);

////////////
/// Events
////////////

#[derive(Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub effect: EffectSpawner,
}

#[derive(Debug)]
pub struct HealEvent {
    pub target: Entity,
    pub effect: EffectSpawner,
}

#[derive(Debug)]
pub struct ParticleEvent {
    pub tile_idx: usize,
    pub effect: EffectSpawner,
}

#[derive(Debug)]
pub struct DeathEvent(pub Entity);

impl_new!(ParticleEvent, tile_idx: usize, effect: EffectSpawner);
impl_new!(DamageEvent, target: Entity, effect: EffectSpawner);
impl_new!(HealEvent, target: Entity, effect: EffectSpawner);
