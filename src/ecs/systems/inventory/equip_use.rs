use super::*;

pub fn equip_use(
    mut commands: Commands,
    // Basic Queries
    player_q: Query<Entity, With<Player>>,
    // Item Queries
    names_q: Query<&Naming>,
    mut equipped_ev: ResMut<Events<WantsToEquipItem>>,
    equippable: Query<&Equippable, Without<Equipped>>,
    equipped: Query<(Entity, &Equipped, &Naming)>,
) {
    for WantsToEquipItem(entity, item) in equipped_ev.drain() {
        let player_entity = player_q.single();
        let can_equip = equippable.get(item).unwrap();
        let target_slot = can_equip.slot;

        // Remove any items the target has in the item's slot
        for (item_entity, already_equipped, name) in &equipped {
            if already_equipped.owner == entity && already_equipped.slot == target_slot {
                commands.entity(item_entity).remove::<Equipped>().insert(InBackpack::new(entity));

                if entity == player_entity {
                    bo_logging::Logger::new().append("You unequip").item_name(&name.0.clone()).log();
                }
            }
        }

        // Wield the item
        commands.entity(item).remove::<InBackpack>();
        commands.entity(item).insert(Equipped::new(entity, target_slot));

        if entity == player_entity {
            let item_name = names_q.get(item).unwrap().0.clone();
            bo_logging::Logger::new().append("You equip").item_name(item_name).log();
        }
    }
}
