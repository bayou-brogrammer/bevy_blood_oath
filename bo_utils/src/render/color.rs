use bracket_terminal::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WHITE_BLACK: ColorPair = ColorPair::new(WHITE, BLACK);
}

pub const BOX_GRAY: (u8, u8, u8) = (153, 153, 153);
pub const LIGHT_RED: (u8, u8, u8) = (255, 204, 203);
