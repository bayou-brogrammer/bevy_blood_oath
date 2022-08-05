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

    // Providing food
    if world.get::<ProvidesFood>(entity).is_some() {
        did_something = true;
        let item_name = world.get::<Naming>(entity).unwrap().0.clone();
        add_effect(creator, EffectType::WellFed, targets.clone());
        bo_logging::Logger::new().append("You eat the").item_name(item_name).log();
    }

    // Healing
    if let Some(heal) = world.get::<ProvidesHealing>(entity) {
        did_something = true;
        add_effect(creator, EffectType::Healing(heal.0), targets.clone());
    }

    // Damage
    if let Some(damage) = world.get::<InflictsDamage>(entity) {
        add_effect(creator, EffectType::Damage(damage.damage), targets.clone());
        did_something = true;
    }

    // Confusion
    if let Some(confusion) = world.get::<Confusion>(entity) {
        add_effect(creator, EffectType::Confusion(confusion.turns), targets.clone());
        did_something = true;
    }

    // Magic mapper
    if world.get::<MagicMapper>(entity).is_some() {
        bo_logging::Logger::new().append("The map is revealed to you!").log();
        let mut runstate = world.resource_mut::<TurnState>();
        *runstate = TurnState::MagicMapReveal(0);
        did_something = true;
    }

    did_something
}
