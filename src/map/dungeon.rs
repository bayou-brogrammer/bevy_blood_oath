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
}
