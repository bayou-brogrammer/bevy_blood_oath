use super::*;

use lazy_static::lazy_static;

////////////////////////////////////////////////////////////////////////////////

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = UI_WIDTH - 1;
pub const LOG_PANEL_HEIGHT: i32 = 7;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = UI_WIDTH - 31;
pub const MAP_PANEL_HEIGHT: i32 = UI_HEIGHT - 8;

// Map Panel
pub const STATS_PANEL_WIDTH: i32 = 30;
pub const STATS_PANEL_HEIGHT: i32 = 8;

pub const EQUIPMENT_PANEL_WIDTH: i32 = 30;
pub const EQUIPMENT_PANEL_HEIGHT: i32 = UI_HEIGHT - LOG_PANEL_HEIGHT - STATS_PANEL_HEIGHT - 1;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT);
    pub static ref LOG_PANEL: Rect =
        Rect::with_size(0, MAP_PANEL_HEIGHT, LOG_PANEL_WIDTH, LOG_PANEL_HEIGHT);
    pub static ref STATS_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, 0, STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT);
    pub static ref EQUIPMENT_PANEL: Rect = Rect::with_size(
        MAP_PANEL_WIDTH,
        STATS_PANEL_HEIGHT,
        EQUIPMENT_PANEL_WIDTH,
        EQUIPMENT_PANEL_HEIGHT
    );
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

pub fn map_label(draw_batch: &mut DrawBatch, world: &mut World) {
    let map = world.resource::<Map>();
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

fn draw_stats(draw_batch: &mut DrawBatch, world: &mut World) {
    let mut stats_q = world.query_filtered::<&CombatStats, With<Player>>();
    for stats in stats_q.iter(world) {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        draw_batch.print_color(Point::new(50, 1), &health, *WHITE_BLACK);
        draw_batch.bar_horizontal(
            Point::new(64, 1),
            14,
            stats.hp,
            stats.max_hp,
            ColorPair::new(RED, BLACK),
        );
    }
}

fn equipped(draw_batch: &mut DrawBatch, world: &mut World) -> i32 {
    let mut equipped_q = world.query::<(&Equipped, &Naming, Option<&MeleePowerBonus>)>();
    if let Some(player_entity) = world.get_resource::<Entity>() {
        let mut y = 13;
        for (equipped_by, name, melee_bonus) in equipped_q.iter(world) {
            if equipped_by.owner == *player_entity {
                let item_name = name.0.clone();

                draw_batch.print_color(
                    Point::new(50, y),
                    &item_name,
                    ColorPair::new(RGB::from_f32(0.5, 1.0, 0.5), BLACK),
                );
                y += 1;

                if let Some(melee_bonus) = melee_bonus {
                    let mut weapon_info = format!("┤ {} ({})", &item_name, melee_bonus.power);
                    weapon_info += " ├";
                    draw_batch.print_color(
                        Point::new(3, LOG_PANEL.y1),
                        &weapon_info,
                        ColorPair::new(YELLOW, BLACK),
                    );
                }
            }
        }

        y
    } else {
        0
    }
}

fn status(draw_batch: &mut DrawBatch, world: &mut World) {
    if let Some(player) = world.get_resource::<Entity>() {
        let hc = world.get::<HungerClock>(*player).unwrap();

        let y = EQUIPMENT_PANEL.y2 - 1;
        match hc.state {
            HungerState::Normal => {}
            HungerState::WellFed => {
                draw_batch.print_color(Point::new(50, y), "Well Fed", ColorPair::new(GREEN, BLACK));
                // y -= 1;
            }
            HungerState::Hungry => {
                draw_batch.print_color(Point::new(50, y), "Hungry", ColorPair::new(ORANGE, BLACK));
                // y -= 1;
            }
            HungerState::Starving => {
                draw_batch.print_color(Point::new(50, y), "Starving", ColorPair::new(RED, BLACK));
                // y -= 1;
            }
        }
    }
}

pub fn render_ui(world: &mut World) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(LAYER_TEXT);

    box_framework(&mut gui_batch);
    map_label(&mut gui_batch, world);
    draw_stats(&mut gui_batch, world);
    equipped(&mut gui_batch, world);
    status(&mut gui_batch, world);
    print_log(LAYER_TEXT, Point::new(1, UI_HEIGHT - LOG_PANEL_HEIGHT + 1));

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}
