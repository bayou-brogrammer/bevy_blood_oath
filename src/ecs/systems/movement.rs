use crate::prelude::*;

pub fn movement(
    map: Res<Map>,
    player: Res<Entity>,
    mut commands: Commands,
    positions: Query<&Point>,
    mut fov_q: Query<&mut FieldOfView>,
    mut door_q: Query<(Entity, &mut Glyph, &Point), With<Door>>,
    mut move_events: ResMut<Events<WantsToMove>>,
) {
    for WantsToMove(entity, destination) in move_events.drain() {
        door_q.iter_mut().filter(|(_, _, p)| **p == destination).for_each(|(door, mut glyph, _)| {
            commands.entity(door).remove::<BlocksVisibility>().remove::<BlocksTile>().insert(Door(true));
            glyph.glyph = to_cp437('/');

            update_fov(entity, &mut fov_q);
        });

        if map.in_bounds(destination) && map.can_enter_tile(destination) {
            commands.entity(entity).insert(destination);

            let pos = positions.get(entity).unwrap();
            let start_idx = map.point2d_to_index(*pos);
            let dest_idx = map.point2d_to_index(destination);

            crate::spatial::move_entity(entity, start_idx, dest_idx);

            update_fov(entity, &mut fov_q);
            if entity == *player {
                commands.insert_resource(destination);
                commands.insert_resource(CameraView::new(destination));
            }
        }
    }
}

fn update_fov(entity: Entity, fov_q: &mut Query<&mut FieldOfView>) {
    if let Ok(mut fov) = fov_q.get_mut(entity) {
        fov.is_dirty = true;
    }
}
