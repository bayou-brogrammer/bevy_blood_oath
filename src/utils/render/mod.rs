use bracket_lib::prelude::*;

mod boxes;
mod color;
mod menu;
mod string;

pub use boxes::*;
pub use color::*;
pub use menu::*;
pub use string::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        batch.print_color(pos, text, color);
    }
}

pub fn clear_all_consoles<C: Into<Vec<usize>>>(ctx: &mut BTerm, consoles: C) {
    let consoles: Vec<usize> = consoles.into();

    for layer in consoles {
        ctx.set_active_console(layer);
        ctx.cls();
    }

    ctx.set_active_console(0);
}
