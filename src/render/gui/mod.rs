use super::*;

////////////////////////////////////////////////////////////////////////////////

// Stats Panel
pub const STATS_PANEL_WIDTH: i32 = 30;
pub const STATS_PANEL_HEIGHT: i32 = 8;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = UI_WIDTH - STATS_PANEL_WIDTH - 1;
pub const MAP_PANEL_HEIGHT: i32 = UI_HEIGHT - 1;

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = MAP_PANEL_WIDTH;
pub const LOG_PANEL_HEIGHT: i32 = 7;

// Equipment Panel
pub const EQUIPMENT_PANEL_WIDTH: i32 = STATS_PANEL_WIDTH;
pub const EQUIPMENT_PANEL_HEIGHT: i32 = UI_HEIGHT - LOG_PANEL_HEIGHT - STATS_PANEL_HEIGHT;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT);
    pub static ref LOG_PANEL: Rect =
        Rect::with_size(0, UI_HEIGHT - LOG_PANEL_HEIGHT, LOG_PANEL_WIDTH, LOG_PANEL_HEIGHT);
    pub static ref STATS_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, 0, STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT);
    pub static ref EQUIPMENT_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, STATS_PANEL_HEIGHT, EQUIPMENT_PANEL_WIDTH, EQUIPMENT_PANEL_HEIGHT);
    pub static ref OVERALL_PANEL: Rect = Rect::with_size(0, 0, UI_WIDTH - 1, UI_HEIGHT - 1);
}

////////////////////////////////////////////////////////////////////////////////

pub fn box_framework(draw_batch: &mut DrawBatch) {
    draw_batch.draw_hollow_box(*STATS_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Top-right panel
    draw_batch.draw_hollow_box(*MAP_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Map box
    draw_batch.draw_hollow_box(*LOG_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Log box
    draw_batch.draw_hollow_box(*EQUIPMENT_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Log box
    draw_batch.draw_hollow_box(*OVERALL_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Overall box

    // Draw box connectors
    draw_batch.set(Point::new(MAP_PANEL_WIDTH, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┬'));
    draw_batch.set(
        Point::new(UI_WIDTH - 1, MAP_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('┤'),
    );
    draw_batch.set(
        Point::new(MAP_PANEL_WIDTH, STATS_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('├'),
    );
    draw_batch.set(
        Point::new(MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('┴'),
    );
    draw_batch.set(
        Point::new(UI_WIDTH - 1, STATS_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('┤'),
    );
}

pub fn labels(draw_batch: &mut DrawBatch, world: &World) {
    let map = world.resource::<Map>();

    // Map Label
    crate::utils::print_label(draw_batch, &map.name, Point::new(0, 0), MAP_PANEL.width(), WHITE, WHITE);

    // Stats
    print_label(draw_batch, "Stats", Point::new(MAP_PANEL.x2, 0), STATS_PANEL.width(), WHITE, WHITE);
    // Equipment
    print_label(
        draw_batch,
        "Equipment",
        Point::new(MAP_PANEL.x2, EQUIPMENT_PANEL.y1),
        STATS_PANEL.width(),
        WHITE,
        WHITE,
    );
}

fn draw_stats(draw_batch: &mut DrawBatch, world: &mut World) {
    let mut stats_q = world.query_filtered::<&CombatStats, With<Player>>();
    for stats in stats_q.iter(world) {
        let health = format!("Health: {}/{}", stats.hp, stats.max_hp);
        let mana = format!("Mana:   {}/{}", 30, 30);
        let xp = format!("Level:  {}", 1);

        let text_x = STATS_PANEL.x1 + 1;
        let bar_x = text_x + 14;

        draw_batch.print_color(Point::new(text_x, 1), &health, ColorPair::new(WHITE, BLACK));
        draw_batch.print_color(Point::new(text_x, 2), &mana, ColorPair::new(WHITE, BLACK));
        draw_batch.print_color(Point::new(text_x, 3), &xp, ColorPair::new(WHITE, BLACK));

        draw_batch.bar_horizontal(
            Point::new(bar_x, 1),
            14,
            stats.hp,
            stats.max_hp,
            ColorPair::new(RED, BLACK),
        );
        draw_batch.bar_horizontal(Point::new(bar_x, 2), 14, 20, 30, ColorPair::new(NAVYBLUE, BLACK));
        let xp_level_start = 0;
        draw_batch.bar_horizontal(
            Point::new(bar_x, 3),
            14,
            0 - xp_level_start,
            1000,
            ColorPair::new(GOLD, BLACK),
        );
    }
}

fn equipped(draw_batch: &mut DrawBatch, world: &mut World) -> i32 {
    let mut equipped_q = world.query::<(&Equipped, &Naming, Option<&MeleePowerBonus>)>();
    if let Some(player_entity) = world.get_resource::<Entity>() {
        let mut y = EQUIPMENT_PANEL.y1 + 1;
        let x = EQUIPMENT_PANEL.x1 + 1;
        for (equipped_by, name, melee_bonus) in equipped_q.iter(world) {
            if equipped_by.owner == *player_entity {
                let item_name = name.0.clone();

                draw_batch.print_color(
                    Point::new(x, y),
                    &item_name,
                    ColorPair::new(RGB::from_f32(0.5, 1.0, 0.5), BLACK),
                );
                y += 1;

                if let Some(melee_bonus) = melee_bonus {
                    let weapon_info = format!("{} ({})", &item_name, melee_bonus.power);
                    print_label(
                        draw_batch,
                        &weapon_info,
                        Point::new(MAP_PANEL.x2, LOG_PANEL.y1),
                        LOG_PANEL.width(),
                        YELLOW,
                        WHITE,
                    )
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

        let x = EQUIPMENT_PANEL.x1 + 1;
        let y = EQUIPMENT_PANEL.y2 - 1;
        match hc.state {
            HungerState::Normal => {}
            HungerState::WellFed => {
                draw_batch.print_color(Point::new(x, y), "Well Fed", ColorPair::new(GREEN, BLACK));
                // y -= 1;
            }
            HungerState::Hungry => {
                draw_batch.print_color(Point::new(x, y), "Hungry", ColorPair::new(ORANGE, BLACK));
                // y -= 1;
            }
            HungerState::Starving => {
                draw_batch.print_color(Point::new(x, y), "Starving", ColorPair::new(RED, BLACK));
                // y -= 1;
            }
        }
    }
}

pub fn render_ui(world: &mut World) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(LAYER_TEXT);

    box_framework(&mut gui_batch);
    labels(&mut gui_batch, world);
    draw_stats(&mut gui_batch, world);
    equipped(&mut gui_batch, world);
    status(&mut gui_batch, world);
    bo_logging::print_log(&mut gui_batch, *LOG_PANEL);

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}
