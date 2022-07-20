use super::*;

pub fn map_indexing(
    mut map: ResMut<Map>,
    blockers: Query<(Entity, &Position, Option<&BlocksTile>), Changed<Position>>,
    dead_q: Query<Entity, Added<Dead>>,
) {
    if blockers.is_empty() && dead_q.is_empty() {
        return;
    }

    map.populate_blocked();
    map.clear_content_index();

    for (entity, pos, blocker) in blockers.iter() {
        let idx = map.point2d_to_index(pos.0);

        // If they block, update the blocking list
        if let Some(_) = blocker {
            map.tiles[idx].blocked = true;
        }

        map.tiles[idx].contents.insert(entity);
    }
}
