use super::*;
use crate::camera::GameCamera;

pub fn ranged_targeting(ctx: &mut BTerm, world: &mut World, range: i32, item: Entity) {
    println!("draw:");
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_PARTICLES);

    draw_batch.print_color(
        Point::new(5, 0),
        "Select Target:",
        ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
    );

    let mut player_q = world.query_filtered::<(Entity, &Position), With<Player>>();
    let available_cells = world.resource_scope(|world, camera: Mut<GameCamera>| -> Vec<Point> {
        let (player, pos) = player_q.iter(world).next().unwrap();
        let fov = world.get::<FieldOfView>(player).unwrap();

        let mut available_cells = Vec::new();
        for pt in fov.visible_tiles.iter() {
            draw_batch.set_bg(camera.world_to_screen(*pt), BLUE);
            // let distance = DistanceAlg::Pythagoras.distance2d(pos.0, *pt);
            // if distance <= range as f32 {
            //     draw_batch.set_bg(camera.world_to_screen_text(*pt), BLUE);
            //     available_cells.push(*pt);
            // }
        }

        available_cells
    });

    // Draw mouse cursor
    let mouse_pos = ctx.mouse_pos();
    let mut valid_target = false;
    for idx in available_cells.iter() {
        if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 {
            valid_target = true;
        }
    }

    let result = if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(CYAN));

        if ctx.left_click {
            ItemMenuResult::Selected(Point::new(mouse_pos.0, mouse_pos.1))
        } else {
            ItemMenuResult::NoResponse
        }
    } else {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(RED));

        if ctx.left_click {
            ItemMenuResult::Cancel
        } else {
            ItemMenuResult::NoResponse
        }
    };

    match result {
        ItemMenuResult::Cancel => world.insert_resource(StateStack::new(TurnState::AwaitingInput)),
        ItemMenuResult::Selected(pt) => {
            let player = player_q.iter(world).next().unwrap();
            world.resource_scope(|world, mut ew: Mut<EventWriter<WantsToUseItem>>| {
                ew.send(WantsToUseItem::new(item, Some(pt), player.0));
                world.insert_resource(StateStack::new(TurnState::PlayerTurn))
            });
        }
        _ => {}
    }

    draw_batch.submit(BATCH_UI).expect("Batch error");
}
