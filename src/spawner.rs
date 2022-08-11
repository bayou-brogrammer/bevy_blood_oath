use crate::prelude::*;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(mut commands: Commands, map_builder: Res<BuilderMap>) {
    let start_pos = map_builder.starting_position.unwrap();

    // Spawn Player
    let player = commands
        .spawn()
        .insert_bundle(PlayerBundle::new(FighterBundle::new(
            FieldOfView::new(8),
            CombatStats::new(30, 30, 2, 5),
        )))
        .insert_bundle(RenderBundle {
            position: start_pos,
            glyph: Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor),
        })
        .insert(Naming("SecBot".to_string()))
        .insert(Description::new("A bot that can attack and move."))
        .insert(Blood(DARK_RED.into()))
        .insert(HungerClock::new(HungerState::WellFed, 20))
        .id();

    commands.insert_resource(player);
    commands.insert_resource(start_pos);
    commands.insert_resource(GameCamera::new(start_pos));

    // spawner::bear_trap(&mut commands, start_pos + Point::new(1, 0));
    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, MAGIC_MAPPING_SCROLL),
            RenderBundle::new(to_cp437(')'), ColorPair::new(CYAN3, BLACK), RenderOrder::Item, start_pos),
        ))
        .insert(Consumable {})
        .insert(MagicMapper {})
        .remove::<Point>()
        .insert(InBackpack { owner: player });

    commands
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Fire"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(ORANGE, BLACK), RenderOrder::Item, start_pos),
        ))
        .insert(Consumable {})
        .insert(Ranged(6))
        .insert(InflictsDamage(6))
        // .insert(AreaOfEffect::new(3))
        .remove::<Point>()
        .insert(InBackpack { owner: player });
}

pub fn spawn_entities(mut commands: Commands, map_builder: Res<BuilderMap>) {
    println!("Spawning entities...: {:?}", map_builder.spawn_list);
    for entity in map_builder.spawn_list.iter() {
        spawner::spawn_entity(&mut commands, &map_builder.map, &(&entity.0, &entity.1));
    }

    commands.remove_resource::<BuilderMap>()
}

fn room_table(map_depth: i32) -> MasterTable { raws::get_spawn_table_for_depth(&RAWS.lock(), map_depth) }
const MAX_MONSTERS: i32 = 10;

/// Fills a room with stuff!
pub fn spawn_room(map: &Map, room: &Rect, map_depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let mut possible_targets: Vec<usize> = Vec::new();
    {
        // Borrow scope - to keep access to the map separated
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx].tile_type == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(&possible_targets, map_depth, spawn_list);
}

/// Fills a region with stuff!
pub fn spawn_region(area: &[usize], map_depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, Option<String>> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let num_spawns =
            i32::min(areas.len() as i32, crate::rng::roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 {
                0usize
            } else {
                (crate::rng::roll_dice(1, areas.len() as i32) - 1) as usize
            };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll());
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for (spawn_idx, spawn_key) in spawn_points.iter() {
        if spawn_key.is_some() {
            spawn_list.push((*spawn_idx, spawn_key.as_ref().unwrap().to_string()));
        }
    }
}

/// Spawns a named entity (name in tuple.1) at the location in (tuple.0)
pub fn spawn_entity(commands: &mut Commands, map: &Map, spawn: &(&usize, &String)) {
    let pt = map.index_to_point2d(*spawn.0);

    let spawn_result = spawn_named_entity(commands, spawn.1, SpawnType::AtPosition(pt));
    if spawn_result.is_some() {
        return;
    }

    println!("WARNING: We don't know how to spawn [{}]!", spawn.1);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system_set(
            GameCondition::Setup,
            SystemSet::new().with_system(spawn_player).with_system(spawn_entities.after(spawn_player)),
        );

        app.add_exit_system(GameCondition::Playing, |mut commands: Commands, q: Query<Entity>| {
            println!("Exiting game");
            q.iter().for_each(|e| {
                commands.entity(e).despawn_recursive();
            });
        });
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
