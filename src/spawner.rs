use crate::prelude::*;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(world: &mut World, start_pos: Point) {
    // Spawn Player
    let player = world
        .spawn()
        .insert_bundle(PlayerBundle::new(FighterBundle::new(
            FieldOfView::new(8),
            CombatStats::new(30, 30, 2, 5),
        )))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor),
        })
        .insert(Naming("SecBot".to_string()))
        .insert(Description::new("A bot that can attack and move."))
        .insert(Blood(DARK_RED.into()))
        .id();

    world.insert_resource(player);
    world.insert_resource(start_pos);
    world.insert_resource(camera::GameCamera::new(start_pos));

    // spawner::confusion_scroll(&mut world, start_pos);
    // spawner::magic_missile_scroll(&mut world, start_pos);
    // spawner::fireball_scroll(&mut world, start_pos);
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
        .add("Dagger", 3)
        .add("Shield", 3)
        .add("Longsword", map_depth - 1)
        .add("Tower Shield", map_depth - 1)
}

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

    spawn_region(map, &possible_targets, map_depth, spawn_list);
}

/// Fills a region with stuff!
pub fn spawn_region(_map: &Map, area: &[usize], map_depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let num_spawns = i32::min(
            areas.len() as i32,
            bo_utils::rng::roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3,
        );
        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 {
                0usize
            } else {
                (bo_utils::rng::roll_dice(1, areas.len() as i32) - 1) as usize
            };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll());
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}

/// Spawns a named entity (name in tuple.1) at the location in (tuple.0)
pub fn spawn_entity(world: &mut World, spawn: &(&usize, &String)) {
    let map = world.resource::<Map>();
    let pt = map.index_to_point2d(*spawn.0);

    match spawn.1.as_ref() {
        "Orc" => orc(world, pt),
        "Dagger" => dagger(world, pt),
        "Shield" => shield(world, pt),
        "Goblin" => goblin(world, pt),
        // "Rations" => rations(world, pt),
        // "Bear Trap" => bear_trap(world, pt),
        "Longsword" => longsword(world, pt),
        "Tower Shield" => tower_shield(world, pt),
        "Health Potion" => health_potion(world, pt),
        "Fireball Scroll" => fireball_scroll(world, pt),
        "Confusion Scroll" => confusion_scroll(world, pt),
        "Magic Missile Scroll" => magic_missile_scroll(world, pt),
        // "Magic Mapping Scroll" => magic_mapping_scroll(world, pt),
        _ => {}
    }
}

fn orc(world: &mut World, pt: Point) {
    monster(world, pt, 157, "Orc", "An ugly powerful orc")
}
fn goblin(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('g'), "Goblin", "An ugly goblin")
}

pub fn monster(world: &mut World, start_pos: Point, glyph: FontCharType, name: &str, desc: &str) {
    world
        .spawn()
        .insert_bundle(MonsterBundle::new(FighterBundle::new(
            FieldOfView::new(8),
            CombatStats::new(16, 16, 1, 4),
        )))
        .insert_bundle(RenderBundle {
            position: Position::new(start_pos),
            glyph: Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor),
        })
        .insert(Naming(name.to_string()))
        .insert(Description::new(desc))
        .insert(Blood(LIGHT_GREEN.into()));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Items
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn health_potion(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Health Potion"),
            RenderBundle::new(to_cp437('¡'), ColorPair::new(MAGENTA, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(ProvidesHealing(10));
}

pub fn magic_missile_scroll(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Magic Missile Scroll"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(Ranged { range: 6 })
        .insert(InflictsDamage { damage: 100 });
}

pub fn fireball_scroll(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Fireball Scroll"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(ORANGE, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(Ranged { range: 6 })
        .insert(InflictsDamage { damage: 20 })
        .insert(AreaOfEffect { radius: 3 });
}

pub fn confusion_scroll(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Confusion Scroll"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(ORANGE, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(Ranged { range: 6 })
        .insert(Confusion { turns: 4 });
}

fn dagger(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Dagger"),
            RenderBundle::new(to_cp437('/'), ColorPair::new(CYAN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Melee })
        .insert(MeleePowerBonus::new(2));
}

fn shield(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Shield"),
            RenderBundle::new(to_cp437('('), ColorPair::new(CYAN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Shield })
        .insert(DefenseBonus::new(1));
}

fn longsword(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Longsword"),
            RenderBundle::new(to_cp437('/'), ColorPair::new(YELLOW, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Melee })
        .insert(MeleePowerBonus::new(4));
}

fn tower_shield(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Tower Shield"),
            RenderBundle::new(to_cp437('('), ColorPair::new(YELLOW, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Equippable { slot: EquipmentSlot::Shield })
        .insert(DefenseBonus::new(3));
}

fn rations(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Rations"),
            RenderBundle::new(to_cp437('%'), ColorPair::new(GREEN, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable)
        .insert(ProvidesFood {});
}

pub fn magic_mapping_scroll(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Scroll of Magic Mapping"),
            RenderBundle::new(to_cp437(')'), ColorPair::new(CYAN3, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Consumable {})
        .insert(MagicMapper {});
}

fn bear_trap(world: &mut World, pt: Point) {
    world
        .spawn()
        .insert_bundle(ItemBundle::new(
            EntityBundle::new(Item, "Bear Trap"),
            RenderBundle::new(to_cp437('^'), ColorPair::new(RED, BLACK), RenderOrder::Item, pt),
        ))
        .insert(Hidden {})
        .insert(EntryTrigger {})
        .insert(InflictsDamage::new(6))
        .insert(SingleActivation {});
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(GameCondition::InGame, |mut commands: Commands, q: Query<Entity>| {
            println!("Exiting game");
            q.iter().for_each(|e| {
                commands.entity(e).despawn_recursive();
            });
        });
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
