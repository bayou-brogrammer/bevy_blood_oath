use super::*;

pub fn menu_option<T: ToString>(
    draw_batch: &mut DrawBatch,
    x: i32,
    y: i32,
    hotkey: FontCharType,
    text: T,
    selected: bool,
) {
    let bg_color = if selected { SELECTED_BG } else { BLACK };

    draw_batch.set(Point::new(x, y), ColorPair::new(WHITE, BLACK), to_cp437('('));
    draw_batch.set(Point::new(x + 1, y), ColorPair::new(WHITE, bg_color), hotkey);
    draw_batch.set(Point::new(x + 2, y), ColorPair::new(WHITE, BLACK), to_cp437(')'));
    draw_batch.print_color(Point::new(x + 5, y), &text.to_string(), ColorPair::new(WHITE, bg_color));
}

pub fn print_label<S: ToString, C: Into<RGBA> + Copy>(
    draw_batch: &mut DrawBatch,
    str: S,
    pos: Point,
    display_width: i32,
    title_color: C,
    divider_color: C,
) {
    let title = str.to_string();
    let name_length = title.len() as i32;
    let x_pos = (display_width / 2) - (name_length / 2);

    // Left Side
    draw_batch.set(Point::new(pos.x + x_pos - 2, pos.y), ColorPair::new(divider_color, BLACK), to_cp437('│'));
    for i in (pos.x + x_pos - 1)..=(pos.x + x_pos) {
        draw_batch.set(Point::new(i, pos.y), ColorPair::new(divider_color, BLACK), to_cp437(' '));
    }
    // Right Side
    draw_batch.set(
        Point::new(pos.x + x_pos + name_length + 1, pos.y),
        ColorPair::new(divider_color, BLACK),
        to_cp437('│'),
    );
    for i in (pos.x + x_pos)..=pos.x + x_pos + name_length {
        draw_batch.set(Point::new(i, pos.y), ColorPair::new(divider_color, BLACK), to_cp437(' '));
    }
    // Title
    draw_batch.print_color(Point::new(pos.x + x_pos, pos.y), &title, ColorPair::new(title_color, BLACK));
}
