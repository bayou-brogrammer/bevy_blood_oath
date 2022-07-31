use super::camera::GameCamera;
use crate::prelude::*;

pub fn movement(
    map: Res<Map>,
    mut commands: Commands,
    positions: Query<&Position>,
    mut move_events: ResMut<Events<WantsToMove>>,
    mut option_q: Query<(Option<&mut FieldOfView>, Option<&Player>)>,
) {
    for WantsToMove(entity, destination) in move_events.drain() {
        if map.in_bounds(destination) && map.can_enter_tile(destination) {
            commands.entity(entity).insert(Position(destination));

            let pos = positions.get(entity).unwrap();
            let start_idx = map.point2d_to_index(pos.0);
            let dest_idx = map.point2d_to_index(destination);

            crate::spatial::move_entity(entity, start_idx, dest_idx);

            if let Ok((fov, player)) = option_q.get_mut(entity) {
                if let Some(mut fov) = fov {
                    fov.is_dirty = true;
                }

                if player.is_some() {
                    commands.insert_resource(destination);
                    commands.insert_resource(GameCamera::new(destination));
                }
            }
        }
    }
}
