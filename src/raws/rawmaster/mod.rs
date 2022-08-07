use super::*;
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
            raws: Raws { items: Vec::new(), mobs: Vec::new(), props: Vec::new() },
        }
    }
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
