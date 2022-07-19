use super::*;
use bracket_lib::prelude::Rect;

mod menus;
pub use menus::*;

mod boxes;
pub use boxes::*;

lazy_static! {
    pub static ref STAT_PANEL_BOX: Rect = Rect::with_exact(81, 0, 111, 30);
    pub static ref LOG_PANEL_BOX: Rect =
        Rect::with_exact(0, SCREEN_HEIGHT - 8, 80, SCREEN_HEIGHT - 1);
    pub static ref MAP_PANEL_WIDTH: usize = SCREEN_WIDTH - STAT_PANEL_BOX.width() as usize;
    pub static ref MAP_PANEL_HEIGHT: usize = SCREEN_HEIGHT - LOG_PANEL_BOX.height() as usize;
}

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        //println!("Batched text[{}] at {:?}", text.to_string(), pos);
        batch.print_color(pos, text, color);
    }
}

pub fn render_panels(batch: &mut DrawBatch) {
    batch.target(LAYER_TEXT); // Draw on the text layer

    // Log Panel
    batch.draw_box(*LOG_PANEL_BOX, ColorPair::new(DARK_GRAY, BLACK));

    // Side Panel
    batch.draw_box(*STAT_PANEL_BOX, ColorPair::new(DARK_GRAY, BLACK));
    batch.print_color_centered_at(
        Point::new(97, 1),
        "SecBot - 2021 7DRL",
        ColorPair::new(WHITE, BLACK),
    );
}

pub fn render_status(batch: &mut DrawBatch, world: &mut World) {
    batch.target(LAYER_TEXT); // Draw on the text layer

    let mut query = world.query_filtered::<&CombatStats, With<Player>>();
    for stats in query.iter(world) {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        batch.print_color(Point::new(82, 3), &health, ColorPair::new(WHITE, BLACK));
        batch.bar_horizontal(
            Point::new(82 + health.len(), 3),
            16,
            stats.hp,
            stats.max_hp,
            ColorPair::new(RED, BLACK),
        );
    }
}
