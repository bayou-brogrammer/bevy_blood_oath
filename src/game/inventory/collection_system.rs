use super::*;

pub fn item_collection(
    mut commands: Commands,
    mut pickup_events: ResMut<Events<WantsToPickupItem>>,
    player_q: Query<Entity, With<Player>>,
    names_q: Query<&Name>,
) {
    for WantsToPickupItem { item, collected_by } in pickup_events.drain() {
        commands.entity(item).remove::<Position>();
        commands.entity(item).insert(InBackpack(collected_by));

        if collected_by == player_q.single() {
            let item_name = names_q.get(item).unwrap();

            crate::gamelog::Logger::new()
                .append("You pick up the")
                .item_name(item_name.0.clone())
                .log();
        }
    }
}
