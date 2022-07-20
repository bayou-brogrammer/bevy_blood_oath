use super::*;

pub fn item_use(
    mut commands: Commands,
    potions: Query<(&Potion, &Name)>,
    mut stats_q: Query<&mut CombatStats>,
    player_q: Query<Entity, With<Player>>,
    mut drink_events: ResMut<Events<WantsToDrinkPotion>>,
) {
    for WantsToDrinkPotion { potion, drinker } in drink_events.drain() {
        if let Ok(potion) = potions.get(potion) {
            if let Ok(mut stats) = stats_q.get_mut(drinker) {
                stats.hp = i32::min(stats.max_hp, stats.hp + potion.0.heal_amount);

                if drinker == player_q.single() {
                    crate::gamelog::Logger::new()
                        .append("You drink the")
                        .item_name(potion.1.clone())
                        .log();
                }
            }
        }

        commands.entity(potion).despawn_recursive();
    }
}
