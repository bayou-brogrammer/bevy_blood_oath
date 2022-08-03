#![allow(dead_code)] //TODO: remove this

use crate::prelude::*;
use std::collections::{HashMap, HashSet};

const POTION_COLORS: &[&str] = &["Red", "Orange", "Yellow", "Green", "Brown", "Indigo", "Violet"];
const POTION_ADJECTIVES: &[&str] =
    &["Swirling", "Effervescent", "Slimey", "Oiley", "Viscous", "Smelly", "Glowing"];

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MasterDungeonMap {
    maps: HashMap<i32, Map>,
    pub identified_items: HashSet<String>,
    pub scroll_mappings: HashMap<String, String>,
    pub potion_mappings: HashMap<String, String>,
}

impl MasterDungeonMap {
    pub fn new() -> MasterDungeonMap {
        MasterDungeonMap {
            maps: HashMap::new(),
            identified_items: HashSet::new(),
            scroll_mappings: HashMap::new(),
            potion_mappings: HashMap::new(),
        }

        // for scroll_tag in crate::raws::get_scroll_tags().iter() {
        //     let masked_name = dm.make_scroll_name();
        //     dm.scroll_mappings.insert(scroll_tag.to_string(), masked_name);
        // }

        // let mut used_potion_names: HashSet<String> = HashSet::new();
        // for potion_tag in crate::raws::get_potion_tags().iter() {
        //     let masked_name = dm.make_potion_name(&mut used_potion_names);
        //     dm.potion_mappings.insert(potion_tag.to_string(), masked_name);
        // }
    }

    pub fn store_map(&mut self, map: &Map) {
        self.maps.insert(map.depth, map.clone());
    }

    pub fn get_map(&self, depth: i32) -> Option<Map> {
        if self.maps.contains_key(&depth) {
            let result = self.maps[&depth].clone();
            Some(result)
        } else {
            None
        }
    }

    pub fn freeze_level_entities(ecs: &mut World) {
        // Obtain ECS access
        let entities = ecs.entities();
        let map_depth = ecs.fetch::<Map>().depth;
        let player_entity = ecs.fetch::<Entity>();

        let mut positions = ecs.write_storage::<Position>();
        let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();

        // Find positions and make OtherLevelPosition
        let mut pos_to_delete: Vec<Entity> = Vec::new();
        for (entity, pos) in (&entities, &positions).join().filter(|(e, _)| *e != *player_entity) {
            if entity != *player_entity {
                other_level_positions
                    .insert(entity, OtherLevelPosition::new(pos.0, map_depth))
                    .expect("Insert fail");
                pos_to_delete.push(entity);
            }
        }

        // Remove positions
        for p in pos_to_delete.iter() {
            positions.remove(*p);
        }
    }

    pub fn level_transition(ecs: &mut World, new_depth: i32, offset: i32) -> Option<Vec<Map>> {
        // Obtain the master dungeon map
        let dungeon_master = ecs.read_resource::<MasterDungeonMap>();

        // Do we already have a map?
        if dungeon_master.get_map(new_depth).is_some() {
            std::mem::drop(dungeon_master);
            MasterDungeonMap::transition_to_existing_map(ecs, new_depth, offset);
            None
        } else {
            std::mem::drop(dungeon_master);
            Some(MasterDungeonMap::transition_to_new_map(ecs, new_depth))
        }
    }

    pub fn thaw_level_entities(ecs: &mut World) {
        // Obtain ECS access
        let entities = ecs.entities();
        let map_depth = ecs.fetch::<Map>().depth;
        let player_entity = ecs.fetch::<Entity>();
        let mut positions = ecs.write_storage::<Position>();
        let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();

        // Find OtherLevelPosition
        let mut pos_to_delete: Vec<Entity> = Vec::new();
        for (entity, pos) in (&entities, &other_level_positions)
            .join()
            .filter(|(entity, pos)| *entity != *player_entity && pos.depth == map_depth)
        {
            positions.insert(entity, Position::new(pos.pt)).expect("Insert fail");
            pos_to_delete.push(entity);
        }

        // Remove positions
        for p in pos_to_delete.iter() {
            other_level_positions.remove(*p);
        }
    }
}

impl MasterDungeonMap {
    fn _make_scroll_name(&self) -> String {
        let length = 4 + bo_utils::rng::roll_dice(1, 4);
        let mut name = "Scroll of ".to_string();

        for i in 0..length {
            if i % 2 == 0 {
                name += match bo_utils::rng::roll_dice(1, 5) {
                    1 => "a",
                    2 => "e",
                    3 => "i",
                    4 => "o",
                    _ => "u",
                }
            } else {
                name += match bo_utils::rng::roll_dice(1, 21) {
                    1 => "b",
                    2 => "c",
                    3 => "d",
                    4 => "f",
                    5 => "g",
                    6 => "h",
                    7 => "j",
                    8 => "k",
                    9 => "l",
                    10 => "m",
                    11 => "n",
                    12 => "p",
                    13 => "q",
                    14 => "r",
                    15 => "s",
                    16 => "t",
                    17 => "v",
                    18 => "w",
                    19 => "x",
                    20 => "y",
                    _ => "z",
                }
            }
        }

        name
    }

    fn _make_potion_name(&self, used_names: &mut HashSet<String>) -> String {
        loop {
            let mut name: String = POTION_ADJECTIVES
                [bo_utils::rng::roll_dice(1, POTION_ADJECTIVES.len() as i32) as usize - 1]
                .to_string();
            name += " ";
            name += POTION_COLORS[bo_utils::rng::roll_dice(1, POTION_COLORS.len() as i32) as usize - 1];
            name += " Potion";

            if !used_names.contains(&name) {
                used_names.insert(name.clone());
                return name;
            }
        }
    }

    fn transition_to_new_map(world: &mut World, new_depth: i32) -> Vec<Map> {
        let mut builder = map_builders::random_builder(1);
        builder.build_map();

        // Add Up Stairs
        if new_depth > 1 {
            let mut map = builder.get_map();
            let up_idx = map.point2d_to_index(builder.get_starting_position());
            map.tiles[up_idx] = GameTile::stairs_up();
        }

        let player_start;
        {
            let mut worldmap_resource = world.write_resource::<Map>();
            *worldmap_resource = builder.get_map();
            player_start = builder.get_starting_position();
        }

        builder.spawn_entities(world);

        // Setup Player Position / FOV
        {
            let player_entity = world.fetch::<Entity>();
            let mut player_pt = world.write_resource::<Point>();
            let mut position_components = world.write_storage::<Position>();

            *player_pt = player_start;
            position_components.insert(*player_entity, Position::new(player_start)).expect("Insert fail");

            // Mark the player's visibility as dirty
            let mut fov_components = world.write_storage::<FieldOfView>();
            let fov = fov_components.get_mut(*player_entity);
            if let Some(fov) = fov {
                fov.is_dirty = true;
            }
        }

        // Setup Camera
        world.insert(GameCamera::new(player_start));

        // Store the newly minted map
        let mut dungeon_master = world.write_resource::<MasterDungeonMap>();
        dungeon_master.store_map(&builder.get_map());

        builder.get_snapshot_history()
    }

    fn transition_to_existing_map(ecs: &mut World, new_depth: i32, offset: i32) {
        let dungeon_master = ecs.read_resource::<MasterDungeonMap>();
        let map = dungeon_master.get_map(new_depth).unwrap();

        let player_entity = ecs.fetch::<Entity>();
        let mut worldmap_resource = ecs.write_resource::<Map>();

        // Find the down stairs and place the player
        let stair_type = if offset < 0 { TileType::DownStairs } else { TileType::UpStairs };

        for (idx, _tile) in map.get_tile_type(stair_type).iter().enumerate() {
            let mut player_position = ecs.write_resource::<Point>();
            *player_position = map.index_to_point2d(idx);

            let mut position_components = ecs.write_storage::<Position>();
            let player_pos_comp = position_components.get_mut(*player_entity);

            if let Some(player_pos_comp) = player_pos_comp {
                player_pos_comp.0 = map.index_to_point2d(idx);
                if new_depth == 1 {
                    player_pos_comp.0.x -= 1;
                }
            }
        }

        *worldmap_resource = map;

        // Mark the player's visibility as dirty
        let mut fov_storage = ecs.write_storage::<FieldOfView>();
        let fov = fov_storage.get_mut(*player_entity);
        if let Some(fov) = fov {
            fov.is_dirty = true;
        }
    }
}
