use crate::prelude::*;

pub mod camera;
pub mod gui;

pub fn clear_all_consoles(ctx: &mut BTerm, consoles: &Vec<usize>) {
    for layer in consoles.iter() {
        ctx.set_active_console(*layer);
        ctx.cls();
    }

    if !consoles.is_empty() {
        ctx.set_active_console(consoles[0])
    }
}

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        //println!("Batched text[{}] at {:?}", text.to_string(), pos);
        batch.print_color(pos, text, color);
    }
}
