use super::*;

pub fn combat(
    stats_query: Query<(&CombatStats, &Naming)>,
    mut attack_events: ResMut<Events<WantsToAttack>>,
    defense_bonus: Query<(Entity, &DefenseBonus, &Equipped)>,
    melee_bonus: Query<(Entity, &MeleePowerBonus, &Equipped)>,
) {
    for WantsToAttack(attacker, victim) in attack_events.drain() {
        if let Ok((attacker_stats, attacker_name)) = stats_query.get(attacker) {
            if attacker_stats.hp > 0 {
                let mut offensive_bonus = 0;
                for (_item_entity, power_bonus, equipped_by) in &melee_bonus {
                    if equipped_by.owner == attacker {
                        offensive_bonus += power_bonus.power;
                    }
                }

                let (target_stats, target_name) = stats_query.get(victim).unwrap();
                if target_stats.hp > 0 {
                    let mut defensive_bonus = 0;
                    for (_item_entity, defense_bonus, equipped_by) in &defense_bonus {
                        if equipped_by.owner == attacker {
                            defensive_bonus += defense_bonus.defense;
                        }
                    }

                    let damage = i32::max(
                        0,
                        (attacker_stats.power + offensive_bonus)
                            - (target_stats.defense + defensive_bonus),
                    );

                    if damage == 0 {
                        bo_logging::Logger::new()
                            .color(CYAN)
                            .append(&attacker_name.0)
                            .color(WHITE)
                            .append("attacks")
                            .color(CYAN)
                            .append(&target_name.0)
                            .color(WHITE)
                            .append("but can't connect.")
                            .log();

                        add_hit_miss_particle(victim);
                    } else {
                        bo_logging::Logger::new()
                            .npc_name(&attacker_name.0)
                            .append("hits")
                            .npc_name(&target_name.0)
                            .append("for")
                            .damage(damage)
                            .append("hp.")
                            .log();

                        add_effect(
                            Some(attacker),
                            EffectType::Damage { amount: damage },
                            Targets::Single { target: victim },
                        );
                    }
                }
            }
        }
    }
}
