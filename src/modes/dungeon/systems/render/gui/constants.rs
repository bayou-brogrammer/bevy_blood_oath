use super::*;

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
