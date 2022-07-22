use super::*;
use bracket_lib::prelude::Rect;
use lazy_static::lazy_static;

mod menus;
pub use menus::*;

////////////////////////////////////////////////////////////////////////////////

// Stats Panel
pub const STAT_PANEL_WIDTH: usize = 32;

// Log Panel
pub const LOG_PANEL_WIDTH: usize = SCREEN_WIDTH - STAT_PANEL_WIDTH;
pub const LOG_PANEL_HEIGHT: usize = 8;

lazy_static! {
    // Log Panel
    pub static ref LOG_PANEL_BOX: Rect = Rect::with_exact(
        0,
        SCREEN_HEIGHT - LOG_PANEL_HEIGHT,
        LOG_PANEL_WIDTH,
        SCREEN_HEIGHT - 1
    );

    // Stats Panel
    pub static ref STAT_PANEL_BOX: Rect = Rect::with_exact(
        SCREEN_WIDTH - STAT_PANEL_WIDTH,
        0,
        SCREEN_WIDTH - 1,
        SCREEN_HEIGHT - 1
    );

    // Map Panel
    pub static ref MAP_PANEL_WIDTH: usize = SCREEN_WIDTH - STAT_PANEL_BOX.width() as usize;
    pub static ref MAP_PANEL_HEIGHT: usize = SCREEN_HEIGHT - LOG_PANEL_BOX.height() as usize;
}

fn render_ui(stats_q: Query<&CombatStats, With<Player>>) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(LAYER_TEXT);

    gui::render_panels(&mut gui_batch);
    gui::render_status(&mut gui_batch, stats_q);
    print_log(&mut gui_batch, Point::new(1, LOG_PANEL_BOX.y1 + 1));

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
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

////////////////////////////////////////////////////////////////////////////////

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        // GUI Ticking Systems
        app.add_system_set(
            ConditionSet::new().run_if(run_in_game_state).with_system(render_ui).into(),
        );

        // GUI Inventory Systems
        app.add_system_set(
            ConditionSet::new()
                .run_if(run_in_stack(TurnState::Inventory))
                .with_system(menus::show_inventory::<{ InventoryMenu::Main as u8 }>)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_if(run_in_stack(TurnState::ShowDropMenu))
                .with_system(menus::show_inventory::<{ InventoryMenu::Drop as u8 }>)
                .into(),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
