use super::*;

pub fn map_indexing(
    mut map: ResMut<Map>,
    blockers: Query<(Entity, &Position, Option<&BlocksTile>), Changed<Position>>,
    dead_q: Query<Entity, Added<Dead>>,
) {
    if blockers.is_empty() && dead_q.is_empty() {
        return;
    }

    map.clear_content_index();
    map.populate_blocked();

    for (entity, pos, blocker) in blockers.iter() {
        let idx = map.point2d_to_index(pos.0);
        spatial::index_entity(entity, idx, blockers.get(entity).is_ok());
    }
}
