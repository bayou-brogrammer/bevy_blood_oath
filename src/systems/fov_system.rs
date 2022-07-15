use super::*;

pub struct FovSystem;

impl<'a> System<'a> for FovSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, pos_storage, mut fov_storage, player_storage) = data;

        for (fov, pos, player) in (&mut fov_storage, &pos_storage, (&player_storage).maybe()).join()
        {
            fov.is_dirty = false;
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, &*map);

            if player.is_some() {
                map.clear_visible();

                fov.visible_tiles.iter().for_each(|pt| {
                    map.set_visibility(*pt);
                });
            }
        }
    }
}
