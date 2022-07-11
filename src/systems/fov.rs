use crate::prelude::*;

#[system]
#[read_component(Position)]
#[write_component(FieldOfView)]
pub fn fov(world: &mut SubWorld, #[resource] map: &mut Map) {
    // Build the player FOV

    let mut views = <(Entity, &Position, &mut FieldOfView)>::query();
    let mut player_q = <Entity>::query().filter(component::<Player>());
    let player = *player_q.iter(world).next().unwrap();

    views
        .iter_mut(world)
        .filter(|(_, _, fov)| fov.is_dirty)
        .for_each(|(entity, pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_current());
            fov.is_dirty = false;

            if *entity == player {
                println!("{:?}", player);

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
        });
}
