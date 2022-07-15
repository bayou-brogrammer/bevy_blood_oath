use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Glyph {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
