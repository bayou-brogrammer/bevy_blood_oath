use super::*;

mod game_menus;
mod title_menus;

pub use game_menus::*;
pub use title_menus::*;

use lazy_static::lazy_static;

////////////////////////////////////////////////////////////////////////////////

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = UI_WIDTH - 1;
pub const LOG_PANEL_HEIGHT: i32 = 10;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = UI_WIDTH - 31;
pub const MAP_PANEL_HEIGHT: i32 = UI_HEIGHT - 10;

// Map Panel
pub const STATS_PANEL_WIDTH: i32 = 30;
pub const STATS_PANEL_HEIGHT: i32 = 8;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT);
    pub static ref LOG_PANEL: Rect =
        Rect::with_size(0, MAP_PANEL_HEIGHT, LOG_PANEL_WIDTH, LOG_PANEL_HEIGHT);
    pub static ref STATS_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, 0, STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT);
    pub static ref OVERALL_PANEL: Rect = Rect::with_size(0, 0, UI_WIDTH - 1, UI_HEIGHT - 1);
}

////////////////////////////////////////////////////////////////////////////////

fn box_framework(draw_batch: &mut DrawBatch) {
    draw_batch.draw_hollow_box(*STATS_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Top-right panel
    draw_batch.draw_hollow_box(*MAP_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Map box
    draw_batch.draw_hollow_box(*LOG_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Log box
    draw_batch.draw_hollow_box(*OVERALL_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Overall box

    // Draw box connectors
    draw_batch.set(Point::new(0, 45), ColorPair::new(BOX_GRAY, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 8), ColorPair::new(BOX_GRAY, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┬'));
    draw_batch.set(Point::new(49, 45), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┴'));
    draw_batch.set(Point::new(79, 8), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┤'));
    draw_batch.set(Point::new(79, 45), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┤'));
}

pub fn map_label(draw_batch: &mut DrawBatch, map: Res<Map>) {
    let name_length = map.name.len() + 2;
    let x_pos = (22 - (name_length / 2)) as i32;
    draw_batch.set(Point::new(x_pos, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┤'));
    draw_batch.set(
        Point::new(x_pos + name_length as i32 - 1, 0),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('├'),
    );
    draw_batch.print_color(Point::new(x_pos + 1, 0), &map.name, *WHITE_BLACK);
}

fn draw_stats(draw_batch: &mut DrawBatch, stats_q: &Query<&CombatStats, With<Player>>) {
    let stats = stats_q.single();

    let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);

    draw_batch.print_color(Point::new(50, 1), &health, *WHITE_BLACK);
    draw_batch.bar_horizontal(Point::new(64, 1), 14, stats.hp, stats.max_hp, ColorPair::new(RED, BLACK));
}

fn render_ui(map: Res<Map>, stats_q: Query<&CombatStats, With<Player>>) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(LAYER_TEXT);

    box_framework(&mut gui_batch);
    map_label(&mut gui_batch, map);
    draw_stats(&mut gui_batch, &stats_q);
    print_log(LAYER_TEXT, Point::new(1, UI_HEIGHT - LOG_PANEL_HEIGHT + 1));

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}

////////////////////////////////////////////////////////////////////////////////

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TitleMenuPlugins);

        // GUI Ticking Systems
        app.add_system_set(
            ConditionSet::new().run_in_state(GameCondition::InGame).with_system(render_ui).into(),
        );

        // GUI Inventory Systems
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::Inventory)
                .with_system(game_menus::show_inventory::<{ InventoryMenu::Main as u8 }>)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::ShowDropMenu)
                .with_system(game_menus::show_inventory::<{ InventoryMenu::Drop as u8 }>)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::ShowRemoveMenu)
                .with_system(game_menus::show_inventory::<{ InventoryMenu::Remove as u8 }>)
                .into(),
        );

        // Targeting
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::Targeting)
                .with_system(game_menus::ranged_targeting)
                .with_system(game_menus::ranged_input)
                .into(),
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
