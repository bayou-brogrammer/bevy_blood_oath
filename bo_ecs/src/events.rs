use crate::prelude::*;
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
pub struct WantsToDrinkPotion {
    pub potion: Entity,
    pub drinker: Entity,
}

#[derive(Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
    pub dropper: Entity,
}

#[derive(Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub effect: EffectSpawner,
}

#[derive(Debug)]
pub struct DeathEvent(pub Entity);
