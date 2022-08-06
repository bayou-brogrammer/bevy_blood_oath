use super::*;

pub fn map_indexing(
    mut map: ResMut<Map>,
    dead_q: Query<Entity, Added<Dead>>,
    blocking_q: Query<(Entity, &Point, Option<&BlocksTile>, Option<&BlocksVisibility>)>,
) {
    if blocking_q.is_empty() && dead_q.is_empty() {
        return;
    }

    map.clear_content_index();
    crate::spatial::populate_blocked_from_map(&map);
    crate::spatial::populate_opaque_from_map(&map);

    for (entity, pos, blocker, opaque) in blocking_q.iter() {
        let idx = map.point2d_to_index(*pos);
        spatial::index_entity(entity, idx, blocker.is_some(), opaque.is_some());
    }
}
