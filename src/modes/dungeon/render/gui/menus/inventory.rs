use super::*;

#[derive(PartialEq, Eq)]
pub enum InventoryMenu {
    Main,
    Drop,
}

impl InventoryMenu {
    pub fn label(&self) -> &'static str {
        match self {
            InventoryMenu::Main => "Inventory",
            InventoryMenu::Drop => "Drop Which Item?",
        }
    }
}

pub fn show_inventory<const MENU_TYPE: InventoryMenu>(
    mut selection: Local<usize>,
    key: Res<Option<VirtualKeyCode>>,
    player: Query<Entity, With<Player>>,
    mut state_stack: ResMut<StateStack<TurnState>>,
    mut drink_event: EventWriter<WantsToDrinkPotion>,
    mut drop_event: EventWriter<WantsToDropItem>,
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
        MENU_TYPE.label(),
        items.len(),
        &items,
        *key,
        *selection,
    ) {
        ItemMenuResult::Cancel => state_stack.set(TurnState::PlayerTurn),
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
            match MENU_TYPE {
                InventoryMenu::Main => {
                    drink_event.send(WantsToDrinkPotion {
                        potion: item,
                        drinker: player,
                    });
                }
                InventoryMenu::Drop => {
                    println!("Dropping {:?}", item);
                    drop_event.send(WantsToDropItem {
                        item,
                        dropper: player,
                    });
                }
            }

            state_stack.set(TurnState::PlayerTurn);
        }
        _ => {} // No Response
    }

    draw_batch.submit(50_000).expect("Batch error"); // On top of everything
}
