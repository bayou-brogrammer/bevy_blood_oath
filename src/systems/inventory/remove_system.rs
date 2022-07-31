use super::*;

pub fn remove_item(mut commands: Commands, mut remove_events: ResMut<Events<WantsToRemoveItem>>) {
    for WantsToRemoveItem(entity, item) in remove_events.drain() {
        commands.entity(item).insert(InBackpack::new(entity)).remove::<Equipped>();
    }
}
