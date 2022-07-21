use bracket_terminal::prelude::{ColorPair, DrawBatch, Point};

mod boxes;
pub use boxes::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        batch.print_color(pos, text, color);
    }
}
