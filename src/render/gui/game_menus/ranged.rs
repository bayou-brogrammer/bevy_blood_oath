use super::*;
use crate::camera::GameCamera;
use std::collections::HashSet;

pub fn ranged_input(
    mut commands: Commands,
    camera: Res<GameCamera>,
    mouse: Res<MousePosition>,
    key: Option<Res<VirtualKeyCode>>,
    left_click: Option<Res<MouseLeftClick>>,
    // Queries
    targeting: Option<Res<Targeting>>,
    player_q: Query<Entity, With<Player>>,
    //Evs
    mut wants_to_use: ResMut<Events<WantsToUseItem>>,
) {
    // Handle Escaping
    if key.as_deref() == Some(&VirtualKeyCode::Escape) {
        commands.insert_resource(TurnState::AwaitingInput);
    }

    if let Some(targeting) = targeting {
        let Targeting { item, range: _ } = *targeting;
        let player = player_q.single();
        let map_mouse_pos = camera.world_to_screen(mouse.pt);

        // Handle Left Mouse || Resturn Key Press
        if key.as_deref() == Some(&VirtualKeyCode::Return) || left_click.is_some() {
            commands.remove_resource::<Targeting>();
            wants_to_use.send(WantsToUseItem(player, item, Some(map_mouse_pos)));
            commands.insert_resource(TurnState::PlayerTurn);
        }
    }
}

pub fn ranged_targeting(
    map: Res<Map>,
    mouse: Res<MousePosition>,
    camera: Res<GameCamera>,
    // Queries
    targeting: Option<Res<Targeting>>,
    item_q: Query<(&Naming, Option<&AreaOfEffect>)>,
    player_q: Query<(&Position, &FieldOfView), With<Player>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_ZERO);

    if let Some(targeting) = targeting {
        let Targeting { range, item } = *targeting;

        let (item_name, item_aoe) = item_q.get(item).unwrap();
        let (player_pos, player_fov) = player_q.single();

        draw_batch.print_color(
            Point::new(2, 2),
            format!("Select Target for {}", item_name.0.clone()),
            ColorPair::new(YELLOW, BLACK),
        );

        let valid_cells = player_fov
            .visible_tiles
            .iter()
            .filter(|pt| DistanceAlg::Pythagoras.distance2d(player_pos.0, **pt) < range as f32)
            .filter(|pt| map.tiles[map.point2d_to_index(**pt)].tile_type == TileType::Floor)
            .copied()
            .collect::<HashSet<Point>>();

        // Draw potential valid cells
        valid_cells.iter().for_each(|pt| {
            let screen_pt = camera.screen_to_world(*pt);
            draw_batch.set_bg(screen_pt, BLUE);
        });

        let mouse_pos = mouse.pt;
        let mouse_map_pos = camera.world_to_screen(mouse.pt);

        // Draw Blast Radius
        if let Some(aoe) = item_aoe {
            if aoe.radius > 0 {
                field_of_view_set(mouse_map_pos, aoe.radius, &*map)
                    .iter()
                    .filter(|pt| map.visible.get_bit(**pt))
                    .for_each(|pt| {
                        let screen_pt = camera.screen_to_world(*pt);
                        draw_batch.set_bg(screen_pt, LIGHT_RED);
                    });
            }
        }

        let is_valid_target = valid_cells.iter().filter(|pt| **pt == mouse_map_pos).count() > 0;

        // Draw Mouse Cursor
        draw_batch.set_bg(mouse_pos, if is_valid_target { GREEN } else { RED });
    }

    draw_batch.submit(BATCH_DECOR).expect("Batch error");
}
