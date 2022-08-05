use super::*;

pub fn item_use(
    map: Res<Map>,
    mut commands: Commands,
    // Basic Queries
    player_q: Query<Entity, With<Player>>,
    // Item Effects
    aoe_item_q: Query<&AreaOfEffect>,
    mut wants_to_use: ResMut<Events<WantsToUseItem>>,
) {
    for WantsToUseItem(entity, item, target) in wants_to_use.drain() {
        let player_entity = player_q.single();

        add_effect(
            Some(entity),
            EffectType::ItemUse(item),
            match target {
                None => Targets::Single(player_entity),
                Some(target) => {
                    if let Ok(aoe) = aoe_item_q.get(item) {
                        Targets::Tiles(aoe_tiles(&*map, target, aoe.radius))
                    } else {
                        Targets::Tile(map.point2d_to_index(target))
                    }
                }
            },
        );

        commands.entity(entity).remove::<WantsToUseItem>();
    }
}
