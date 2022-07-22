use crate::prelude::*;
use bevy_ecs::prelude::*;
use bracket_geometry::prelude::Point;

pub fn entity_position(entity: Entity, positions: &Query<&Position>) -> Option<Point> {
    // entity.get_component::<Position>()
    if let Ok(pos) = positions.get(entity) {
        Some(pos.0)
    } else {
        None
    }
}
