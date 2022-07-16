use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderOrder {
    Actor,
    Item,
    Corpse,
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Glyph {
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub render_order: RenderOrder,
}

impl Glyph {
    pub fn new(glyph: FontCharType, color: ColorPair, render_order: RenderOrder) -> Self {
        Glyph {
            glyph,
            color,
            render_order,
        }
    }
}
