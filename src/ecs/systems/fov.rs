use super::*;

pub fn fov_system(
    mut map: ResMut<Map>,
    mut views: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
) {
    for (pos, mut fov, player) in views.iter_mut() {
        if fov.is_dirty {
            fov.is_dirty = false;
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, map.as_ref());

            if player.is_some() {
                map.clear_visible();

                fov.visible_tiles.iter().for_each(|pt| {
                    map.set_revealed_and_visible(*pt);
                });
            }
        };
    }
}
