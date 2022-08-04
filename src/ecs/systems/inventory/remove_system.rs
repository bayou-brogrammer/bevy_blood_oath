use super::*;

pub fn remove_item(
    mut commands: Commands,
    // Basic Queries
    names_q: Query<&Naming>,
    player_q: Query<Entity, With<Player>>,
    mut remove_events: ResMut<Events<WantsToRemoveItem>>,
) {
    for WantsToRemoveItem(entity, item) in remove_events.drain() {
        commands.entity(item).insert(InBackpack::new(entity)).remove::<Equipped>();

        if player_q.get(entity).is_ok() {
            let item_name = names_q.get(item).unwrap().0.clone();
            bo_logging::Logger::new().append("You unequip").item_name(item_name).log();
        }
    }
}
