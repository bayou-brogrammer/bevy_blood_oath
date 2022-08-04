use crate::prelude::*;

mod boxes;
pub mod color;
mod menus;

pub use boxes::*;
pub use color::*;
pub use menus::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        batch.print_color(pos, text, color);
    }
}
