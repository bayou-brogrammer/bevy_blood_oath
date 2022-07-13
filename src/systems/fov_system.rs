use super::*;

pub struct FovSystem;

impl<'a> System<'a> for FovSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Door>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, pos_storage, mut fov_storage, player_storage, door_storage) = data;

        for (entity, fov, pos, player) in (
            &entities,
            &mut fov_storage,
            &pos_storage,
            (&player_storage).maybe(),
        )
            .join()
        {
            fov.is_dirty = false;
            fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_current());

            if player.is_some() {
                let current_layer = map.get_current_mut();

                current_layer.clear_visible();
                fov.visible_tiles.iter().for_each(|pt| {
                    if current_layer.in_bounds(*pt) {
                        let idx = current_layer.point2d_to_index(*pt);
                        current_layer.revealed[idx] = true;
                        current_layer.visible[idx] = true;
                    }
                });
            }
        }
    }
}
