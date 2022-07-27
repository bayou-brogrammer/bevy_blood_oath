use super::*;
use crate::camera::GameCamera;
use std::collections::HashSet;

pub fn ranged_targeting(
    map: Res<Map>,
    mouse: Res<Mouse>,
    mut commands: Commands,
    camera: Res<GameCamera>,
    key: Res<Option<VirtualKeyCode>>,
    // Queries
    targeting: Option<Res<Targeting>>,
    item_q: Query<(&Naming, Option<&AreaOfEffect>)>,
    player_q: Query<(Entity, &Position, &FieldOfView), With<Player>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_MAP);

    if let Some(targeting) = targeting {
        let Targeting { range, item } = *targeting;

        let (item_name, item_aoe) = item_q.get(item).unwrap();
        let (player_entity, player_pos, player_fov) = player_q.single();

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
            .map(|pt| *pt)
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
        let action = *key == Some(VirtualKeyCode::Return)
            || *key == Some(VirtualKeyCode::Escape)
            || mouse.left_click;

        // Draw Mouse Cursor
        draw_batch.set_bg(mouse_pos, if is_valid_target { GREEN } else { RED });

        let result = if action {
            if *key == Some(VirtualKeyCode::Escape) {
                ItemMenuResult::Cancel
            } else {
                if is_valid_target {
                    ItemMenuResult::Selected(mouse_map_pos)
                } else {
                    ItemMenuResult::NoResponse
                }
            }
        } else {
            ItemMenuResult::NoResponse
        };

        let mut did_action = false;
        match result {
            ItemMenuResult::Cancel => {
                commands.insert_resource(TurnState::AwaitingInput);
                did_action = true
            }
            ItemMenuResult::Selected(pt) => {
                commands.entity(player_entity).insert(WantsToUseItem::new(targeting.item, Some(pt)));
                did_action = true
            }
            _ => {}
        }

        if did_action {
            commands.remove_resource::<Targeting>();
            commands.insert_resource(TurnState::PlayerTurn);
        }
    }

    draw_batch.submit(BATCH_DECOR).expect("Batch error");
}
