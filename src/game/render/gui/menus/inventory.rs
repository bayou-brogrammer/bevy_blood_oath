use super::*;

pub fn show_inventory(
    mut commands: Commands,
    mut selection: Local<usize>,
    key: Res<Option<VirtualKeyCode>>,
    player: Query<Entity, With<Player>>,
    mut drink_event: EventWriter<WantsToDrinkPotion>,
    items_q: Query<(Entity, &Naming, &InBackpack), With<Item>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_TEXT);

    let player = player.single();

    let mut items: Vec<(Entity, String)> = Vec::new();
    items_q
        .iter()
        .filter(|(_, _, backpack)| backpack.0 == player)
        .for_each(|(item, item_name, _)| items.push((item, item_name.0.clone())));

    match item_result_menu(
        &mut draw_batch,
        "Inventory",
        items.len(),
        &items,
        *key,
        *selection,
    ) {
        ItemMenuResult::Cancel => commands.insert_resource(TurnState::AwaitingInput),
        ItemMenuResult::UpSelection => {
            if *selection > 0 {
                *selection -= 1;
            }
        }
        ItemMenuResult::DownSelection => {
            if *selection < items.len() - 1 {
                *selection += 1;
            }
        }
        ItemMenuResult::Selected(item) => {
            drink_event.send(WantsToDrinkPotion {
                potion: item,
                drinker: player,
            });
            commands.insert_resource(TurnState::PlayerTurn)
        }
        _ => {} // No Response
    }

    draw_batch.submit(50_000).expect("Batch error"); // On top of everything
}
