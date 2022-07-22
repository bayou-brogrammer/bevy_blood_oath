use super::*;

pub fn item_trigger(
    mut commands: Commands,
    consumables: Query<&Consumable>,
    provides_healing: Query<&ProvidesHealing>,
    mut item_events: ResMut<Events<ItemTrigger>>,
) {
    for ItemTrigger { item, creator, targets } in item_events.drain() {
        let mut did_something = false;

        // Healing
        if let Ok(heal) = provides_healing.get(item) {
            did_something = true;
            add_effect(creator, EffectType::Healing { amount: heal.0 }, targets.clone());
        }

        // If it was a consumable, then it gets deleted
        if did_something && consumables.get(item).is_ok() {
            commands.entity(item).despawn();
        }
    }
}
