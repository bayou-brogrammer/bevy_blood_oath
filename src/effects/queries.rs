use crate::prelude::*;
use bracket_geometry::prelude::Point;

pub fn entity_position(world: &mut World, entity: Entity) -> Option<usize> {
    if let Some(pos) = world.get::<Position>(entity) {
        let map = world.get_resource::<Map>().unwrap();
        Some(map.point2d_to_index(pos.0))
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
