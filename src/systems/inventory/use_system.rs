use super::*;

pub fn item_use(
    map: Res<Map>,
    mut commands: Commands,
    // Basic Queries
    names_q: Query<&Naming>,
    consumables_q: Query<&Consumable>,
    mut stats_q: Query<&mut CombatStats>,
    player_q: Query<Entity, With<Player>>,
    // Item Effects?
    aoe_item_q: Query<&AreaOfEffect>,
    confusion_q: Query<(&Confusion, &Naming)>,
    healing_q: Query<(&ProvidesHealing, &Naming)>,
    damage_q: Query<(&InflictsDamage, &Naming)>,
    mut use_events: ResMut<Events<WantsToUseItem>>,
) {
    for WantsToUseItem { item, target, creator } in use_events.drain() {
        let player_entity = player_q.single();
        let mut used_item = true;

        // Targeting
        let mut targets: Vec<Entity> = Vec::new();
        match target {
            None => targets.push(player_entity),
            Some(target) => {
                match aoe_item_q.get(item) {
                    Err(_) => {
                        // Single target in tile
                        let idx = map.point2d_to_index(target);
                        crate::spatial::for_each_tile_content(idx, |e| targets.push(e));
                    }
                    Ok(aoe) => {
                        // AoE
                        let blast_tiles = queries::aoe_tiles(&*map, target, aoe.radius);
                        for tile_idx in blast_tiles.iter() {
                            crate::spatial::for_each_tile_content(*tile_idx, |e| targets.push(e));
                        }
                    }
                }
            }
        }

        // If it heals, apply the healing
        if let Ok((healer, name)) = healing_q.get(item) {
            used_item = false;

            for target in targets.iter() {
                if let Ok(mut stats) = stats_q.get_mut(*target) {
                    stats.hp = i32::min(stats.max_hp, stats.hp + healer.0);

                    if creator == player_entity {
                        bo_logging::Logger::new()
                            .append("You use the")
                            .item_name(name.0.clone())
                            .append("healing")
                            .healing(healer.0)
                            .append("hp.")
                            .log()
                    }

                    used_item = true;
                }
            }
        }

        // If it inflicts damage, apply it to the target cell
        if let Ok((damage, name)) = damage_q.get(item) {
            used_item = false;

            for mob in targets.iter() {
                add_effect(
                    Some(creator),
                    EffectType::Damage { amount: damage.damage },
                    Targets::Single { target: *mob },
                );

                if *mob == player_entity {
                    let mob_name = names_q.get(*mob).unwrap();

                    bo_logging::Logger::new()
                        .append("You use")
                        .item_name(name.0.clone())
                        .append("on")
                        .npc_name(mob_name.0.clone())
                        .append("inflicting")
                        .damage(damage.damage)
                        .append("hp.")
                        .log()
                }

                used_item = true;
            }
        }

        if let Ok((confusion, name)) = confusion_q.get(item) {
            used_item = false;

            for mob in targets.iter() {
                commands.entity(*mob).insert(Confusion { turns: confusion.turns });

                if *mob == player_entity {
                    let mob_name = names_q.get(*mob).unwrap();

                    bo_logging::Logger::new()
                        .append("You use")
                        .item_name(name.0.clone())
                        .append("on")
                        .npc_name(mob_name.0.clone())
                        .append("confusing them")
                        .log();
                }
            }
        }

        // If its a consumable, we delete it on use
        if used_item {
            if let Ok(_) = consumables_q.get(item) {
                commands.entity(item).despawn_recursive();
            }
        }
    }
}
