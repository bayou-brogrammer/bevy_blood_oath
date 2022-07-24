use super::*;

pub fn item_use(
    map: Res<Map>,
    mut commands: Commands,
    // Basic Queries
    player_q: Query<Entity, With<Player>>,
    // Item Effects?
    aoe_item_q: Query<&AreaOfEffect>,
    wants_to_use: Query<(Entity, &WantsToUseItem)>,
) {
    for (entity, WantsToUseItem { item, target }) in wants_to_use.iter() {
        let player_entity = player_q.single();

        add_effect(
            Some(entity),
            EffectType::ItemUse { item: *item },
            match target {
                None => Targets::Single { target: player_entity },
                Some(target) => {
                    if let Ok(aoe) = aoe_item_q.get(*item) {
                        Targets::Tiles { tiles: aoe_tiles(&*map, *target, aoe.radius) }
                    } else {
                        Targets::Tile { tile_idx: map.point2d_to_index(*target) }
                    }
                }
            },
        );

        commands.entity(entity).remove::<WantsToUseItem>();
    }
}
