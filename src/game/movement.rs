use crate::prelude::*;

pub fn movement(
    mut map: ResMut<Map>,
    mut commands: Commands,
    positions: Query<&Position>,
    mut move_events: ResMut<Events<WantsToMove>>,
    mut option_q: Query<(Option<&mut FieldOfView>, Option<&BlocksTile>)>,
) {
    for WantsToMove {
        entity,
        destination,
    } in move_events.drain()
    {
        if map.in_bounds(destination) && map.can_enter_tile(destination) {
            if let Ok((fov, blocker)) = option_q.get_mut(entity) {
                if let Some(mut fov) = fov {
                    fov.is_dirty = true;
                }

                if let Some(_) = blocker {
                    let pos = positions.get(entity).unwrap();
                    map.update_blocked(pos.0, destination);
                }
            }

            commands.entity(entity).insert(Position(destination));
        }
    }
}
