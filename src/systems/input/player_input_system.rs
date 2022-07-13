#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::prelude::*;
use std::collections::HashSet;

pub struct PlayerInputSystem {}

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Key>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, TurnState>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Door>,
    );

    fn run(
        &mut self,
        (
            entities,
            key,
            mut map,
            mut turn_state,
            mut pos_storage,
            mut fov_storage,
            player_storage,
            door_storage,
        ): Self::SystemData,
    ) {
        if let Some(key) = key.0 {
            let delta = match key {
                VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::Down | VirtualKeyCode::A => Point::new(0, 1),
                VirtualKeyCode::Left | VirtualKeyCode::S => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
                _ => Point::new(0, 0),
            };

            let mut result = TurnState::WaitingForInput;
            let mut doors_to_delete = HashSet::new();

            for (pos, fov, _) in (&mut pos_storage, &mut fov_storage, &player_storage).join() {
                let new_pos = pos.pt + delta;
                let new_idx = map.get_current().point2d_to_index(new_pos);

                if !map.get_current().tiles[new_idx].blocked {
                    pos.pt = new_pos;
                    fov.is_dirty = true;
                    result = TurnState::Ticking;
                } else if map.get_current().is_door[new_idx] {
                    map.get_current_mut().open_door(new_idx);
                    doors_to_delete.insert(map.get_current().index_to_point2d(new_idx));
                }
            }

            #[cfg(not(feature = "parallel"))]
            for (entity, pos, _) in (&entities, &pos_storage, &door_storage).join() {
                if pos.layer == map.current_layer && doors_to_delete.contains(&pos.pt) {
                    entities.delete(entity).unwrap();
                }
            }

            #[cfg(feature = "parallel")]
            (&entities, &pos_storage, &door_storage)
                .par_join()
                .for_each(|(entity, pos, _)| {
                    if pos.layer == map.current_layer && doors_to_delete.contains(&pos.pt) {
                        entities.delete(entity).unwrap();
                    }
                });

            *turn_state = result;
        }
    }
}
