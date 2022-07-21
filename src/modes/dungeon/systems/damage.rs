use super::*;

pub fn damage_system(
    mut commands: Commands,
    mut stats_q: Query<(&mut CombatStats, &Naming, Option<&Player>)>,
    mut damage_events: ResMut<Events<SufferDamage>>,
    glyphs: Query<&Glyph>,
) {
    for SufferDamage { victim, damage } in damage_events.drain() {
        if let Ok((mut stats, name, player)) = stats_q.get_mut(victim) {
            stats.hp -= damage;

            // Kill! >:)
            if stats.hp < 1 {
                if player.is_none() {
                    commands.entity(victim).remove_bundle::<MonsterBundle>();

                    if let Ok(g) = glyphs.get(victim) {
                        commands.entity(victim).insert(Glyph {
                            glyph: g.glyph,
                            color: ColorPair::new(DARK_RED, DARK_GRAY),
                            render_order: RenderOrder::Corpse,
                        });
                    }

                    commands
                        .entity(victim)
                        .insert(Naming(format!("Dead {}", name.0)))
                        .insert(Dead);
                }
            }
        }
    }
}
