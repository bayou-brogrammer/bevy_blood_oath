use super::*;

pub fn damage_system(
    mut commands: Commands,
    mut stats_q: Query<(&mut CombatStats, &Naming, Option<&Player>)>,
    mut damage_events: ResMut<Events<SufferDamage>>,
) {
    for SufferDamage { victim, damage } in damage_events.drain() {
        if let Ok((mut stats, name, player)) = stats_q.get_mut(victim) {
            stats.hp -= damage;

            // Kill! >:)
            if stats.hp < 1 && player.is_none() {
                commands.entity(victim).remove_bundle::<MonsterBundle>();
                commands.entity(victim).insert_bundle(DeadBundle {
                    name: Naming(format!("Dead {}", name.0)),
                    glyph: Glyph::new(
                        to_cp437('%'),
                        ColorPair::new(GRAY, BLACK),
                        RenderOrder::Corpse,
                    ),
                });
            }
        }
    }
}
