use super::*;

pub fn inflict_damage(mut damage_events: ResMut<Events<DamageEvent>>, mut stats_q: Query<&mut CombatStats>) {
    for DamageEvent { target, effect } in damage_events.drain() {
        if let EffectType::Damage { amount } = effect.effect_type {
            if let Ok(mut stats) = stats_q.get_mut(target) {
                stats.hp -= amount;

                add_damage_particle(target);

                if stats.hp < 1 {
                    add_effect(effect.creator, EffectType::EntityDeath, Targets::Single { target });
                }
            }
        }
    }
}

pub fn death(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    info_q: Query<(&Glyph, &Naming)>,
    mut death_events: ResMut<Events<DeathEvent>>,
) {
    for DeathEvent(victim) in death_events.drain() {
        if !player.get(victim).is_ok() {
            commands.entity(victim).remove_bundle::<MonsterBundle>();

            let (glyph, name) = info_q.get(victim).unwrap();

            commands.entity(victim).insert(Glyph {
                glyph: glyph.glyph,
                color: ColorPair::new(DARK_RED, DARK_GRAY),
                render_order: RenderOrder::Corpse,
            });

            commands.entity(victim).insert(Naming(format!("Dead {}", name.0))).insert(Dead);
        }
    }
}

pub fn heal_damage(mut stats_q: Query<&mut CombatStats>, mut heal_events: ResMut<Events<HealEvent>>) {
    for HealEvent { target, effect } in heal_events.drain() {
        if let Ok(mut stats) = stats_q.get_mut(target) {
            if let EffectType::Healing { amount } = effect.effect_type {
                stats.hp = i32::min(stats.max_hp, stats.hp + amount);

                bo_logging::Logger::new()
                    .append("You heal")
                    .append_with_color(format!("{}", amount), GREEN)
                    .append("hp")
                    .log();

                // add_effect(
                //     None,
                //     EffectType::Particle {
                //         glyph: rltk::to_cp437('‼'),
                //         fg: rltk::RGB::named(rltk::GREEN),
                //         bg: rltk::RGB::named(rltk::BLACK),
                //         lifespan: 200.0,
                //     },
                //     Targets::Single { target },
                // );
            }
        }
    }
}
