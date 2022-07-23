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
    mut commands: Commands,
    mut selection: Local<usize>,
    ranged_items: Query<&Ranged>,
    key: Res<Option<VirtualKeyCode>>,
    player: Query<Entity, With<Player>>,
    mut use_event: EventWriter<WantsToUseItem>,
    mut drop_event: EventWriter<WantsToDropItem>,
    items_q: Query<(Entity, &Naming, &InBackpack), With<Item>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player = player.single();

    let mut items: Vec<(Entity, String)> = Vec::new();
    items_q
        .iter()
        .filter(|(_, _, backpack)| backpack.0 == player)
        .for_each(|(item, item_name, _)| items.push((item, item_name.0.clone())));

    let menu_type = InventoryMenu::menu_type(MENU_TYPE);
    match item_result_menu(&mut draw_batch, menu_type.label(), items.len() as i32, &items, *key, *selection) {
        ItemMenuResult::Cancel => commands.insert_resource(TurnState::PlayerTurn),
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
                    if let Ok(r) = ranged_items.get(item) {
                        commands.insert_resource(Targeting::new(item, r.range));
                        commands.insert_resource(TurnState::Targeting);
                        return;
                    } else {
                        use_event.send(WantsToUseItem { item, target: None, creator: player });
                    }
                }
                InventoryMenu::Drop => {
                    drop_event.send(WantsToDropItem { item, dropper: player });
                }
            }

            commands.insert_resource(TurnState::PlayerTurn);
        }
        _ => {} // No Response
    }

    draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
}
