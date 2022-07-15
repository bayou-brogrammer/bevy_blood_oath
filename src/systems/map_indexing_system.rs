use super::{BlocksTile, Map, Position};
use bracket_lib::prelude::Algorithm2D;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, position, blockers) = data;

        // map.populate_blocked();
        // map.clear_content_index();
        for (entity, pos, blocker) in (&entities, &position, (&blockers).maybe()).join() {
            let idx = map.point2d_to_index(pos.0);

            // If they block, update the blocking list
            if let Some(_) = blocker {
                map.tiles[idx].blocked = true;
            }

            // Push the entity to the appropriate index slot. It's a Copy
            // type, so we don't need to clone it (we want to avoid moving it out of the ECS!)
            // map.tile_content[idx].push(entity);
            if let Some(content) = map.tiles[idx].contents.as_mut() {
                content.push(entity);
            } else {
                map.tiles[idx].contents = Some(vec![entity]);
            }
        }
    }
}
