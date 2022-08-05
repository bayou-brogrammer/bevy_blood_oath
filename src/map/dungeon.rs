#![allow(dead_code)] //TODO: remove this

use bevy::ecs::system::CommandQueue;

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

    pub fn level_transition(ecs: &mut World, new_depth: i32, offset: i32) -> Option<Vec<Map>> {
        // Obtain the master dungeon map
        let dungeon_master = ecs.resource::<MasterDungeonMap>();

        // Do we already have a map?
        if dungeon_master.get_map(new_depth).is_some() {
            MasterDungeonMap::transition_to_existing_map(ecs, new_depth, offset);
            None
        } else {
            Some(MasterDungeonMap::transition_to_new_map(ecs, new_depth))
        }
    }

    pub fn freeze_level_entities(world: &mut World) {
        // Obtain ECS access
        let mut positions = world.query::<(Entity, &Position)>();
        let map_depth = world.resource::<Map>().depth;
        let player_entity = world.resource::<Entity>();

        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, world);

        // Find positions and make OtherLevelPosition
        for (entity, pos) in positions.iter(world) {
            if entity != *player_entity {
                commands.entity(entity).remove::<Position>();
                commands.entity(entity).insert(OtherLevelPosition::new(pos.0, map_depth));
            }
        }

        queue.apply(world);
    }

    pub fn thaw_level_entities(world: &mut World) {
        // Obtain ECS access
        let mut other_positions = world.query::<(Entity, &OtherLevelPosition)>();
        let map_depth = world.resource::<Map>().depth;
        let player_entity = world.resource::<Entity>();

        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, world);

        // Find OtherLevelPosition
        for (entity, pos) in other_positions
            .iter(world)
            .filter(|(entity, pos)| *entity != *player_entity && pos.depth == map_depth)
        {
            commands.entity(entity).insert(Position::new(pos.pt));
            commands.entity(entity).remove::<OtherLevelPosition>();
        }

        queue.apply(world);
    }
}

impl MasterDungeonMap {
    fn _make_scroll_name(&self) -> String {
        let length = 4 + crate::rng::roll_dice(1, 4);
        let mut name = "Scroll of ".to_string();

        for i in 0..length {
            if i % 2 == 0 {
                name += match crate::rng::roll_dice(1, 5) {
                    1 => "a",
                    2 => "e",
                    3 => "i",
                    4 => "o",
                    _ => "u",
                }
            } else {
                name += match crate::rng::roll_dice(1, 21) {
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
                [crate::rng::roll_dice(1, POTION_ADJECTIVES.len() as i32) as usize - 1]
                .to_string();
            name += " ";
            name += POTION_COLORS[crate::rng::roll_dice(1, POTION_COLORS.len() as i32) as usize - 1];
            name += " Potion";

            if !used_names.contains(&name) {
                used_names.insert(name.clone());
                return name;
            }
        }
    }

    fn transition_to_new_map(world: &mut World, new_depth: i32) -> Vec<Map> {
        let mut builder = map_builders::level_builder(1, 80, 50);
        builder.build_map();

        world.insert_resource(builder.build_data.clone());

        // Add Up Stairs
        if new_depth > 1 {
            if let Some(pos) = &builder.build_data.starting_position {
                let up_idx = builder.build_data.map.point2d_to_index(*pos);
                builder.build_data.map.tiles[up_idx] = GameTile::stairs_up();
            }
        }

        let mapgen_history = builder.build_data.history.clone();
        {
            let mut worldmap_resource = world.resource_mut::<Map>();
            *worldmap_resource = builder.build_data.map.clone();
        }

        // Store the newly minted map
        let mut dungeon_master = world.resource_mut::<MasterDungeonMap>();
        dungeon_master.store_map(&builder.build_data.map);

        mapgen_history
    }

    fn transition_to_existing_map(ecs: &mut World, new_depth: i32, offset: i32) {
        let dungeon_master = ecs.resource::<MasterDungeonMap>();
        let map = dungeon_master.get_map(new_depth).unwrap();
        let player = *ecs.resource::<Entity>();

        // Find the down stairs and place the player
        let stair_type = if offset < 0 { TileType::DownStairs } else { TileType::UpStairs };
        {
            for (idx, _tile) in map.get_tile_type(stair_type).iter().enumerate() {
                let mut player_position = ecs.resource_mut::<Point>();
                *player_position = map.index_to_point2d(idx);

                if let Some(mut player_pos_comp) = ecs.get_mut::<Position>(player) {
                    player_pos_comp.0 = map.index_to_point2d(idx);
                    if new_depth == 1 {
                        player_pos_comp.0.x -= 1;
                    }
                }
            }
        }

        let mut worldmap_resource = ecs.resource_mut::<Map>();
        *worldmap_resource = map;

        if let Some(mut fov) = ecs.get_mut::<FieldOfView>(player) {
            fov.is_dirty = true;
        }
    }
}
