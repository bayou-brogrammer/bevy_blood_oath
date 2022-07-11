use crate::prelude::*;

pub mod gui;
pub mod tooltips;

use tooltips::*;

pub fn build_render_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(render_ui_skeleton_system())
        .add_system(render_glyphs_system())
        .add_system(render_tooltips_system())
        .add_system(render_map_system())
        .build()
}

#[system]
pub fn render_ui_skeleton() {
    let mut batch = DrawBatch::new();

    batch.draw_hollow_box(
        Rect::with_size(0, 0, WIDTH + 1, HEIGHT + 1),
        ColorPair::new(GRAY, BLACK),
    );
    batch.print_color(
        Point::new(2, 0),
        "┤ SecBot 2021 7DRL ├",
        ColorPair::new(WHITE, BLACK),
    );
    batch.draw_hollow_box(
        Rect::with_size(WIDTH + 1, 0, 30, HEIGHT + 1),
        ColorPair::new(GRAY, BLACK),
    );
    batch.set(
        Point::new(WIDTH + 1, 0),
        ColorPair::new(GRAY, BLACK),
        to_cp437('┬'),
    );
    batch.set(
        Point::new(WIDTH + 1, HEIGHT + 1),
        ColorPair::new(GRAY, BLACK),
        to_cp437('┴'),
    );

    batch.submit(5000).expect("Failed to submit draw batch");
}

#[system(par_for_each)]
pub fn render_glyphs(#[resource] map: &Map, pos: &Position, glyph: &Glyph) {
    // if pos.is_none() {
    //     return;
    // }

    let mut batch = DrawBatch::new();

    // if let Some(pos) = pos {
    if pos.layer == map.current_layer {
        let idx = map.get_current().point2d_to_index(pos.pt);
        if map.get_current().visible[idx] {
            batch.set(
                Point::new(pos.pt.x + 1, pos.pt.y + 1),
                glyph.color,
                glyph.glyph,
            );
        }
    }
    // }

    batch.submit(5000).expect("Failed to submit draw batch");
}

#[system]
pub fn render_map(#[resource] map: &Map) {
    map.get_current().render();
}

pub fn modal(ctx: &mut BTerm, title: &str, body: &str) -> NewState {
    let mut draw_batch = DrawBatch::new();
    draw_batch.draw_double_box(Rect::with_size(19, 14, 71, 12), ColorPair::new(CYAN, BLACK));
    let mut buf = TextBuilder::empty();
    buf.ln()
        .fg(YELLOW)
        .bg(BLACK)
        .centered(title)
        .fg(CYAN)
        .bg(BLACK)
        .ln()
        .ln()
        .line_wrap(body)
        .ln()
        .ln()
        .fg(YELLOW)
        .bg(BLACK)
        .centered("PRESS ENTER TO CONTINUE")
        .reset();

    let mut block = TextBlock::new(20, 15, 70, 11);
    block.print(&buf).expect("Overflow occurred");
    block.render_to_draw_batch(&mut draw_batch);
    draw_batch.submit(0).expect("Batch error");

    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Return => NewState::Wait,
            VirtualKeyCode::Space => NewState::Wait,
            _ => NewState::NoChange,
        }
    } else {
        NewState::NoChange
    }
}

pub fn game_over_left(ctx: &mut BTerm) -> NewState {
    ctx.cls();
    ctx.print(
        1,
        1,
        "Game over. You left the map. Haven't written the stuff to show here.",
    );
    ctx.print(
        1,
        2,
        "You need to refresh or reload. Haven't done restarting yet.",
    );
    NewState::NoChange
}
