use crate::prelude::*;

pub fn triggers(
    map: Res<Map>,
    area_of_effect: Query<&AreaOfEffect>,
    positions_q: Query<(Entity, &Point)>,
    triggers: Query<(Entity, Option<&Naming>, &Point), With<EntryTrigger>>,
) {
    for (entity, position) in positions_q.iter() {
        for (entity_id, name, _) in triggers.iter().filter(|(_, _, pos)| **pos == *position) {
            if entity != entity_id {
                // We triggered it
                if let Some(name) = name {
                    bo_logging::Logger::new().item_name(&name.0).append("triggers!").log();
                }

                // Call the effects system
                add_effect(
                    Some(entity),
                    EffectType::TriggerFire(entity_id),
                    if let Ok(aoe) = area_of_effect.get(entity_id) {
                        Targets::Tiles(aoe_tiles(&*map, *position, aoe.radius))
                    } else {
                        Targets::Tile(map.point2d_to_index(*position))
                    },
                );
            }
        }
    }
}
