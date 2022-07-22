use super::*;

pub fn item_use(
    map: Res<Map>,
    aoe_item: Query<(&AreaOfEffect)>,
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
                Some(target) => {
                    if let Ok(aoe) = aoe_item.get(item) {
                        Targets::Tiles { tiles: queries::aoe_tiles(&*map, target, aoe.radius) }
                    } else {
                        Targets::Tile { tile_idx: map.point2d_to_index(target) }
                    }
                }
            },
        );
    }
}
