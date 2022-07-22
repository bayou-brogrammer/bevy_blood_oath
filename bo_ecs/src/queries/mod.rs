use crate::prelude::*;
use bevy_ecs::prelude::*;
use bo_map::prelude::Map;
use bracket_algorithm_traits::prelude::Algorithm2D;
use bracket_geometry::prelude::Point;
use bracket_pathfinding::prelude::field_of_view_set;

pub fn entity_position(entity: Entity, positions: &Query<&Position>) -> Option<Point> {
    if let Ok(pos) = positions.get(entity) {
        Some(pos.0)
    } else {
        None
    }
}

pub fn aoe_tiles(map: &Map, target: Point, radius: i32) -> Vec<usize> {
    let blast_tiles = field_of_view_set(target, radius, &*map);
    let mut result = Vec::new();

    for t in blast_tiles.iter() {
        result.push(map.point2d_to_index(*t));
    }

    result
}
