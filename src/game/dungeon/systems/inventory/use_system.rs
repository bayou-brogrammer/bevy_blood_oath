use super::*;

pub fn item_use(
    mut commands: Commands,
    potions: Query<(&Potion, &Naming)>,
    mut stats_q: Query<&mut CombatStats>,
    player_q: Query<Entity, With<Player>>,
    mut drink_events: ResMut<Events<WantsToDrinkPotion>>,
) {
    for WantsToDrinkPotion { potion, drinker } in drink_events.drain() {
        if let Ok((potion, potion_name)) = potions.get(potion) {
            if let Ok(mut stats) = stats_q.get_mut(drinker) {
                stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);

                if drinker == player_q.single() {
                    crate::gamelog::Logger::new()
                        .append("You drink the")
                        .item_name(potion_name.0.clone())
                        .log();
                }
            }
        }

        commands.entity(potion).despawn_recursive();
    }
}
