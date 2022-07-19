use super::*;

pub fn map_indexing(
    mut map: ResMut<Map>,
    blockers: Query<(Entity, &Position, Option<&BlocksTile>)>,
) {
    map.populate_blocked();
    map.clear_content_index();

    for (entity, pos, blocker) in blockers.iter() {
        let idx = map.point2d_to_index(pos.0);

        // If they block, update the blocking list
        if let Some(_) = blocker {
            map.tiles[idx].blocked = true;
        }

        // Push the entity to the appropriate index slot. It's a Copy
        // type, so we don't need to clone it (we want to avoid moving it out of the ECS!)
        // map.tile_content[idx].push(entity);
        map.tiles[idx].contents.insert(entity);
    }
}
