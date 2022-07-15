use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
#[storage(DenseVecStorage)]
pub struct Glyph {
    pub glyph: FontCharType,
    pub color: ColorPair,
}

impl Glyph {
    pub fn new(glyph: FontCharType, color: ColorPair) -> Self {
        Glyph { glyph, color }
    }
}
