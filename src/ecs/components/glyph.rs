use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Deserialize, Serialize)]
pub enum RenderOrder {
    Particle,
    Actor,
    Item,
    Corpse,
}

#[derive(Debug, PartialEq, Component, Copy, Clone, Serialize, Deserialize)]
pub struct Glyph {
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub render_order: RenderOrder,
}

impl Glyph {
    pub fn new(glyph: FontCharType, color: ColorPair, render_order: RenderOrder) -> Self {
        Glyph { glyph, color, render_order }
    }
}
