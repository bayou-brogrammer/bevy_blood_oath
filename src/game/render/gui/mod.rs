use super::*;
use bracket_lib::prelude::Rect;

mod menus;
pub use menus::*;

mod boxes;
pub use boxes::*;

mod constants;
pub use constants::*;

////////////////////////////////////////////////////////////////////////////////

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        // GUI Ticking Systems
        app.add_system_set_to_stage(
            GameStage::Render,
            ConditionSet::new().with_system(render_ui).into(),
        );

        // GUI Inventory Systems
        app.add_system_set_to_stage(
            GameStage::Render,
            ConditionSet::new()
                // .run_if_resource_equals(TurnState::ShowInventory)
                .run_in_state(TurnState::ShowInventory)
                .with_system(menus::show_inventory)
                .into(),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

fn render_ui(stats_q: Query<&CombatStats, With<Player>>) {
    let mut gui_batch = DrawBatch::new();

    gui::render_panels(&mut gui_batch);
    gui::render_status(&mut gui_batch, stats_q);
    gamelog::print_log(&mut gui_batch, Point::new(1, LOG_PANEL_BOX.y1 + 1));

    gui_batch.submit(40_000).expect("Batch error"); // On top of everything
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

pub fn render_status(batch: &mut DrawBatch, stats_q: Query<&CombatStats, With<Player>>) {
    batch.target(LAYER_TEXT); // Draw on the text layer

    let stats = stats_q.single();
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
