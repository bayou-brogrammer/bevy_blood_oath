use super::*;

pub fn item_trigger(world: &mut World, creator: Option<Entity>, item: Entity, targets: &Targets) {
    // Use the item via the generic system
    let did_something = event_trigger(world, creator, item, targets);

    // If it was a consumable, then it gets deleted
    if did_something && world.get::<Consumable>(item).is_some() {
        world.despawn(item);
    }
}

fn event_trigger(world: &mut World, creator: Option<Entity>, entity: Entity, targets: &Targets) -> bool {
    let mut did_something = false;

    // Healing
    if let Some(heal) = world.get::<ProvidesHealing>(entity) {
        did_something = true;
        add_effect(creator, EffectType::Healing { amount: heal.0 }, targets.clone());
    }

    // Damage
    if let Some(damage) = world.get::<InflictsDamage>(entity) {
        add_effect(creator, EffectType::Damage { amount: damage.damage }, targets.clone());
        did_something = true;
    }

    // Confusion
    if let Some(confusion) = world.get::<Confusion>(entity) {
        add_effect(creator, EffectType::Confusion { turns: confusion.turns }, targets.clone());
        did_something = true;
    }

    did_something
}
