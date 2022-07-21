use super::*;

#[derive(PartialEq, Eq)]
pub enum InventoryMenu {
    Main = 0,
    Drop = 1,
}

impl InventoryMenu {
    pub fn menu_type(menu_type: u8) -> InventoryMenu {
        match menu_type {
            1 => InventoryMenu::Drop,
            _ => InventoryMenu::Main,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            InventoryMenu::Main => "Inventory",
            InventoryMenu::Drop => "Drop Which Item?",
        }
    }
}

// pub fn show_inventory<const MENU_TYPE: InventoryMenu>(
pub fn show_inventory<const MENU_TYPE: u8>(
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

    let menu_type = InventoryMenu::menu_type(MENU_TYPE);
    match item_result_menu(
        &mut draw_batch,
        menu_type.label(),
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
            match menu_type {
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
