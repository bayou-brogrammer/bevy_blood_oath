use crate::prelude::*;
use bracket_geometry::prelude::Point;

// Differently from the source project, which uses resources, we use Bevy's messaging system for move
// and attack messages.
// In the context of this project, it's a bit more ergonomic, but in larger ones, there advantages are
// more significant.
// Watch out! Events persist for two frames, which in this design is not a problem, but it's something
// important to know.

#[derive(Debug)]
pub struct WantsToMove(pub Entity, pub Point);

#[derive(Debug)]
pub struct WantsToAttack(pub Entity, pub Entity);

#[derive(Debug)]
pub struct WantsToPickupItem(pub Entity, pub Entity);

#[derive(Debug, Clone)]
pub struct WantsToDropItem(pub Entity, pub Entity);

#[derive(Debug, Clone)]
pub struct WantsToRemoveItem(pub Entity, pub Entity);

#[derive(Debug, Clone)]
pub struct WantsToEquipItem(pub Entity, pub Entity);

#[derive(Debug, Component)]
pub struct WantsToUseItem(pub Entity, pub Entity, pub Option<Point>);
