use super::*;

pub fn item_drop(
    mut commands: Commands,
    names_q: Query<&Naming>,
    location_q: Query<(&Position, Option<&Player>)>,
    mut drop_events: ResMut<Events<WantsToDropItem>>,
) {
    for WantsToDropItem { item, dropper } in drop_events.drain() {
        let (dropped_pos, player) = location_q.get(dropper).unwrap();

        commands
            .entity(item)
            .insert(Position(dropped_pos.0))
            .remove::<InBackpack>();

        if player.is_some() {
            let item_name = names_q.get(item).unwrap().0.clone();
            crate::gamelog::Logger::new()
                .append("You drop the")
                .item_name(item_name)
                .log();
        }
    }
}