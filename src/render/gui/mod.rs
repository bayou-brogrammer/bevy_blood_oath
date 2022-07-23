use super::*;
use bracket_lib::prelude::Rect;

mod menus;
pub use menus::*;

use lazy_static::lazy_static;

////////////////////////////////////////////////////////////////////////////////

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = SCREEN_WIDTH - 1;
pub const LOG_PANEL_HEIGHT: i32 = 14;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = 49;
pub const MAP_PANEL_HEIGHT: i32 = 45;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, 49, 45);
    pub static ref LOG_PANEL: Rect = Rect::with_size(0, 45, 79, 14);
}

////////////////////////////////////////////////////////////////////////////////

fn box_framework(draw_batch: &mut DrawBatch) {
    let box_gray: RGB = RGB::from_hex("#999999").expect("Oops");

    draw_batch.draw_hollow_box(Rect::with_size(0, 0, 79, 59), ColorPair::new(box_gray, BLACK)); // Overall box
    draw_batch.draw_hollow_box(*MAP_PANEL, ColorPair::new(box_gray, BLACK)); // Map box
    draw_batch.draw_hollow_box(*LOG_PANEL, ColorPair::new(box_gray, BLACK)); // Log box
    draw_batch.draw_hollow_box(Rect::with_size(49, 0, 30, 8), ColorPair::new(box_gray, BLACK)); // Top-right panel

    // Draw box connectors
    draw_batch.set(Point::new(0, 45), ColorPair::new(box_gray, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 8), ColorPair::new(box_gray, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 0), ColorPair::new(box_gray, BLACK), to_cp437('┬'));
    draw_batch.set(Point::new(49, 45), ColorPair::new(box_gray, BLACK), to_cp437('┴'));
    draw_batch.set(Point::new(79, 8), ColorPair::new(box_gray, BLACK), to_cp437('┤'));
    draw_batch.set(Point::new(79, 45), ColorPair::new(box_gray, BLACK), to_cp437('┤'));
}

pub fn map_label(draw_batch: &mut DrawBatch, map: Res<Map>) {
    let box_gray: RGB = RGB::from_hex("#999999").expect("Oops");

    let name_length = map.name.len() as i32;
    // let x_pos = (22 - (name_length / 2)) as i32;
    let x_pos = (MAP_PANEL.width() / 2) - (name_length / 2);

    // Left Side
    draw_batch.set(Point::new(x_pos - 2, 0), ColorPair::new(box_gray, BLACK), to_cp437('├'));
    // Right Side
    draw_batch.set(Point::new(x_pos + name_length + 1, 0), ColorPair::new(box_gray, BLACK), to_cp437('┤'));
    draw_batch.print_color(Point::new(x_pos, 0), &map.name, ColorPair::new(WHITE, BLACK));
}

fn draw_stats(draw_batch: &mut DrawBatch, stats_q: &Query<&CombatStats, With<Player>>) {
    let stats = stats_q.single();

    let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);

    draw_batch.print_color(Point::new(50, 1), &health, ColorPair::new(WHITE, BLACK));
    draw_batch.bar_horizontal(Point::new(64, 1), 14, stats.hp, stats.max_hp, ColorPair::new(RED, BLACK));
}

fn render_ui(map: Res<Map>, stats_q: Query<&CombatStats, With<Player>>) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(0);

    box_framework(&mut gui_batch);
    map_label(&mut gui_batch, map);
    draw_stats(&mut gui_batch, &stats_q);
    print_log(LAYER_LOG, Point::new(1, 23));

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}

////////////////////////////////////////////////////////////////////////////////

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        // GUI Ticking Systems
        app.add_system_set(ConditionSet::new().with_system(render_ui).into());

        // GUI Inventory Systems
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::Inventory)
                .with_system(menus::show_inventory::<{ InventoryMenu::Main as u8 }>)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::ShowDropMenu)
                .with_system(menus::show_inventory::<{ InventoryMenu::Drop as u8 }>)
                .into(),
        );

        // Targeting
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::Targeting)
                .with_system(menus::ranged_targeting)
                .into(),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
