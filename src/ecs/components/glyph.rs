use crate::prelude::*;
use bracket_terminal::prelude::{ColorPair, FontCharType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderOrder {
    Particle,
    Actor,
    Item,
    Corpse,
}

#[derive(Debug, PartialEq, Component)]
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
