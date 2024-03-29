use super::*;

pub fn inflict_damage(world: &mut World, effect: &EffectSpawner, target: Entity) {
    if let Some(mut stats) = world.get_mut::<CombatStats>(target) {
        if let EffectType::Damage(amount) = effect.effect_type {
            stats.hp -= amount;

            add_damage_particle(target);

            if stats.hp < 1 {
                add_effect(effect.creator, EffectType::EntityDeath, Targets::Single(target));
            }
        }
    }

    if let Some(blood) = world.get::<Blood>(target) {
        add_effect(None, EffectType::Bloodstain(blood.0), Targets::Single(target));
    }
}

pub fn bloodstain(world: &mut World, tile_idx: usize, blood_color: &RGB) {
    let mut map = world.resource_mut::<Map>();
    map.bloodstains.insert(tile_idx, *blood_color);
}

pub fn death(world: &mut World, _effect: &EffectSpawner, target: Entity) {
    if let Some(pos) = entity_position(world, target) {
        crate::spatial::remove_entity(target, pos as usize);
    }

    if world.get::<Player>(target).is_none() {
        let mut entity = world.entity_mut(target);

        entity.remove_bundle::<MonsterBundle>();

        if let Some(mut glyph) = entity.get_mut::<Glyph>() {
            glyph.render_order = RenderOrder::Corpse;
            glyph.color = ColorPair::new(DARK_GRAY, BLACK);
        }

        if let Some(name) = entity.get::<Naming>() {
            entity.insert(Naming(format!("Dead {}", name.0))).insert(Dead);
        }
    }
}

pub fn heal_damage(world: &mut World, effect: &EffectSpawner, target: Entity) {
    if let Some(mut stats) = world.get_mut::<CombatStats>(target) {
        if let EffectType::Healing(amount) = effect.effect_type {
            stats.hp = i32::min(stats.max_hp, stats.hp + amount);

            bo_logging::Logger::new()
                .append("You heal")
                .append_with_color(format!("{}", amount), GREEN)
                .append("hp")
                .log();

            add_heal_particle(target);
        }
    }
}

pub fn add_confusion(world: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Confusion(turns) = &effect.effect_type {
        world.entity_mut(target).insert(Confusion { turns: *turns });
    }
}
