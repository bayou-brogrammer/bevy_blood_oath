use crate::prelude::*;

mod colony_info;
pub use colony_info::*;

mod queries;
pub use queries::*;

mod skeleton;
pub use skeleton::*;

mod status;
pub use status::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        //println!("Batched text[{}] at {:?}", text.to_string(), pos);
        batch.print_color(pos, text, color);
    }
}
