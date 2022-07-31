use super::*;

#[derive(PartialEq, Eq)]
pub enum InventoryMenu {
    Main = 0,
    Drop = 1,
    Remove = 2,
}

impl InventoryMenu {
    pub fn menu_type(menu_type: u8) -> InventoryMenu {
        match menu_type {
            1 => InventoryMenu::Drop,
            2 => InventoryMenu::Remove,
            _ => InventoryMenu::Main,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            InventoryMenu::Main => "Inventory",
            InventoryMenu::Drop => "Drop Which Item?",
            InventoryMenu::Remove => "Remove Which Item?",
        }
    }
}

// pub fn show_inventory<const MENU_TYPE: InventoryMenu>(
pub fn show_inventory<const MENU_TYPE: u8>(
    mut commands: Commands,
    mut selection: Local<usize>,
    ranged_items: Query<&Ranged>,
    equippable_items: Query<&Equippable>,
    key: Option<Res<VirtualKeyCode>>,
    player: Query<Entity, With<Player>>,
    mut drop_event: EventWriter<WantsToDropItem>,
    mut equip_event: EventWriter<WantsToEquipItem>,
    mut remove_event: EventWriter<WantsToRemoveItem>,
    backpack_q: Query<(Entity, &Naming, &InBackpack), With<Item>>,
    equipped_q: Query<(Entity, &Naming, &Equipped), With<Item>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_ZERO);

    let player = player.single();
    let menu_type = InventoryMenu::menu_type(MENU_TYPE);

    let mut items: Vec<(Entity, String)> = Vec::new();
    match menu_type {
        InventoryMenu::Main | InventoryMenu::Drop => backpack_q
            .iter()
            .filter(|(_, _, backpack)| backpack.owner == player)
            .for_each(|(item, item_name, _)| items.push((item, item_name.0.clone()))),
        InventoryMenu::Remove => equipped_q
            .iter()
            .filter(|(_, _, backpack)| backpack.owner == player)
            .for_each(|(item, item_name, _)| items.push((item, item_name.0.clone()))),
    }

    match item_result_menu(
        &mut draw_batch,
        (MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT * 2),
        menu_type.label(),
        items.len() as i32,
        &items,
        key.as_deref(),
        *selection,
    ) {
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
            *selection = 0;

            match menu_type {
                InventoryMenu::Main => {
                    if let Ok(r) = ranged_items.get(item) {
                        commands.insert_resource(Targeting::new(item, r.range));
                        commands.insert_resource(TurnState::Targeting);
                        return;
                    } else if equippable_items.get(item).is_ok() {
                        equip_event.send(WantsToEquipItem(player, item));
                    } else {
                        commands.entity(player).insert(WantsToUseItem(item, None));
                    }
                }
                InventoryMenu::Drop => {
                    drop_event.send(WantsToDropItem(player, item));
                }
                InventoryMenu::Remove => {
                    remove_event.send(WantsToRemoveItem(player, item));
                }
            }

            commands.insert_resource(TurnState::PlayerTurn);
        }
        _ => {} // No Response
    }

    draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
}
