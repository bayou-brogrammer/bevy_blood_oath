use super::*;

pub fn combat(
    stats_query: Query<(&CombatStats, &Naming)>,
    mut attack_events: ResMut<Events<WantsToAttack>>,
) {
    for WantsToAttack { victim, attacker } in attack_events.drain() {
        if let Ok((attacker_stats, attacker_name)) = stats_query.get(attacker) {
            if attacker_stats.hp > 0 {
                let (target_stats, target_name) = stats_query.get(victim).unwrap();

                if target_stats.hp > 0 {
                    let damage = i32::max(0, attacker_stats.power - target_stats.defense);

                    if damage == 0 {
                        bo_logging::Logger::new()
                            .npc_name(&attacker_name.0)
                            .append("atacks")
                            .npc_name(&target_name.0)
                            .append("but can't connect.")
                            .log();
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
