use super::*;
use crate::{ecs::*, MasterTable};
use std::collections::HashMap;

mod load;
mod parse;
mod spawn;

pub use load::*;
pub use parse::*;
pub use spawn::*;

pub struct RawMaster {
    raws: Raws,
    mob_index: HashMap<String, usize>,
    item_index: HashMap<String, usize>,
    prop_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            mob_index: HashMap::new(),
            item_index: HashMap::new(),
            prop_index: HashMap::new(),
            raws: Raws { items: Vec::new(), mobs: Vec::new(), props: Vec::new(), spawn_table: Vec::new() },
        }
    }
}

pub fn get_spawn_table_for_depth(raws: &RawMaster, depth: i32) -> MasterTable {
    let available_options: Vec<&SpawnTableEntry> =
        raws.raws.spawn_table.iter().filter(|a| depth >= a.min_depth && depth <= a.max_depth).collect();

    let mut rt = MasterTable::new();
    for e in available_options.iter() {
        let mut weight = e.weight;

        if e.add_map_depth_to_weight.is_some() {
            weight += depth;
        }

        rt.add(e.name.clone(), weight, raws);
    }

    rt
}

pub fn get_renderable_component(glyph: &RawGlyph) -> crate::ecs::Glyph {
    let fg = RGB::from_hex(&glyph.fg).expect("Invalid RGB");
    let bg = RGB::from_hex(&glyph.bg).expect("Invalid RGB");

    crate::ecs::Glyph {
        glyph: to_cp437(glyph.glyph.chars().next().unwrap()),
        color: ColorPair::new(fg, bg),
        render_order: glyph.order,
    }
}

fn find_slot_for_equippable_item(tag: &str, raws: &RawMaster) -> EquipmentSlot {
    if !raws.item_index.contains_key(tag) {
        panic!("Trying to equip an unknown item: {}", tag);
    }

    let item_index = raws.item_index[tag];
    let item = &raws.raws.items[item_index];

    if let Some(_wpn) = &item.weapon {
        return EquipmentSlot::Melee;
    } else if let Some(_wearable) = &item.shield {
        // return string_to_slot(&wearable.slot);
    }

    panic!("Trying to equip {}, but it has no slot tag.", tag);
}
