use bracket_lib::terminal::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use ron::de::from_bytes;
use serde::Deserialize;
use std::collections::HashMap;

mod rawmaster;
mod templates;

pub use rawmaster::*;
pub use templates::*;

embedded_resource!(RAW_ITEM_FILE, "../../resources/raws/items.ron");
embedded_resource!(RAW_MOB_FILE, "../../resources/raws/mobs.ron");
embedded_resource!(RAW_PROP_FILE, "../../resources/raws/props.ron");
embedded_resource!(RAW_SPAWN_TABLE_FILE, "../../resources/raws/spawn_table.ron");

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

#[derive(Deserialize, Debug)]
pub struct Raws {
    pub mobs: Vec<RawMob>,
    pub props: Vec<RawProp>,
    pub items: Vec<RawItem>,
    pub spawn_table: Vec<SpawnTableEntry>,
}

impl Raws {
    fn load_raw<'a, T: serde::Deserialize<'a>>(raw_data: &'static [u8]) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        match from_bytes::<T>(raw_data) {
            Ok(template) => template,
            Err(e) => panic!("Unable to load template: {}", e),
        }
    }

    fn _load_file<'a, T: serde::Deserialize<'a>>(file_path: &str) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        let raw_data = EMBED.lock().get_resource(file_path.to_string()).unwrap();
        match from_bytes::<T>(raw_data) {
            Ok(template) => template,
            Err(e) => panic!("Unable to load template: {}", e),
        }
    }
}

pub fn load_raws() {
    link_resource!(RAW_ITEM_FILE, "resources/raws/items.ron");
    link_resource!(RAW_MOB_FILE, "resources/raws/mobs.ron");
    link_resource!(RAW_PROP_FILE, "resources/raws/props.ron");
    link_resource!(RAW_SPAWN_TABLE_FILE, "resources/raws/spawn_table.ron");

    let mobs = Raws::load_raw::<Vec<RawMob>>(RAW_MOB_FILE);
    let items = Raws::load_raw::<Vec<RawItem>>(RAW_ITEM_FILE);
    let props = Raws::load_raw::<Vec<RawProp>>(RAW_PROP_FILE);
    let spawn_table = Raws::load_raw::<Vec<SpawnTableEntry>>(RAW_SPAWN_TABLE_FILE);

    RAWS.lock().load(Raws { items, mobs, props, spawn_table });
}
