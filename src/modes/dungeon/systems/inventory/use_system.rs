use super::*;

pub fn item_use(
    mut commands: Commands,
    items: Query<(&Consumable, &Naming)>,
    mut stats_q: Query<&mut CombatStats>,
    player_q: Query<Entity, With<Player>>,
    mut use_events: ResMut<Events<WantsToUseItem>>,
) {
    for WantsToUseItem { item, target, creator } in use_events.drain() {
        let player_entity = player_q.single();
        add_effect(
            Some(creator),
            EffectType::ItemUse { item },
            match target {
                None => Targets::Single { target: player_entity },
                // TODO: Fix once aoe is implemented
                _ => Targets::Single { target: player_entity }, // Some(target) => {
                                                                //     if let Some(aoe) = aoe.get(useitem.item) {
                                                                //         Targets::Tiles {
                                                                //             tiles: aoe_tiles(&*map, target, aoe.radius),
                                                                //         }
                                                                //     } else {
                                                                //         Targets::Tile {
                                                                //             tile_idx: map.xy_idx(target.x, target.y) as i32,
                                                                //         }
                                                                //     }
                                                                // }
            },
        );
    }
}
