use crate::camera::GameCamera;

use super::*;

pub fn ranged_targeting(
    mouse: Res<Mouse>,
    mut commands: Commands,
    camera: Res<GameCamera>,
    targeting: Option<Res<Targeting>>,
    mut use_item_event: EventWriter<WantsToUseItem>,
    player_q: Query<(Entity, &Position, &FieldOfView), With<Player>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_MAP);

    if let Some(targeting) = targeting {
        draw_batch.print_color(Point::new(5, 0), "Select Target:", ColorPair::new(YELLOW, BLACK));

        let (player_entity, player_pos, player_fov) = player_q.single();
        let mut available_cells = Vec::new();

        for idx in player_fov.visible_tiles.iter() {
            let distance = DistanceAlg::Pythagoras.distance2d(player_pos.0, *idx);
            if distance <= targeting.range as f32 {
                let screen_pt = camera.screen_to_world(*idx);
                draw_batch.set_bg(screen_pt, BLUE);
                available_cells.push(*idx);
            }
        }

        // Draw mouse cursor
        let mouse_pos = mouse.pt;
        let mouse_map_pos = camera.world_to_screen(mouse.pt) + Point::new(-1, -1);

        let mut valid_target = false;
        for idx in available_cells.iter() {
            if idx.x == mouse_map_pos.x && idx.y == mouse_map_pos.y {
                valid_target = true;
            }
        }

        let result = if valid_target {
            draw_batch.set_bg(mouse_pos, CYAN);

            if mouse.left_click {
                ItemMenuResult::Selected(mouse_map_pos)
            } else {
                ItemMenuResult::NoResponse
            }
        } else {
            draw_batch.set_bg(mouse_pos, RED);

            if mouse.left_click {
                ItemMenuResult::Cancel
            } else {
                ItemMenuResult::NoResponse
            }
        };

        let mut did_action = false;
        match result {
            ItemMenuResult::Cancel => {
                commands.insert_resource(TurnState::AwaitingInput);
                did_action = true
            }
            ItemMenuResult::Selected(pt) => {
                use_item_event.send(WantsToUseItem::new(targeting.item, Some(pt), player_entity));
                did_action = true
            }
            _ => {}
        }

        if did_action {
            commands.remove_resource::<Targeting>();
            commands.insert_resource(TurnState::PlayerTurn);
        }
    }

    draw_batch.submit(BATCH_UI).expect("Batch error");
}
