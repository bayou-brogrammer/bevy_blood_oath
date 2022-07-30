use super::*;

pub fn remove_item(mut commands: Commands, mut remove_events: ResMut<Events<WantsToRemoveItem>>) {
    for WantsToRemoveItem { item, remover } in remove_events.drain() {
        commands.entity(item).insert(InBackpack::new(remover)).remove::<Equipped>();
    }
}
