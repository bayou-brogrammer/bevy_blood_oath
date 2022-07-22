pub mod spawner;

mod effects;
mod render;
mod resources;
mod rng;
mod setup;
mod systems;
mod turn;

mod prelude {
    // Bevy
    pub use bevy::ecs::event::Events;
    pub use bevy::prelude::*;
    pub use bevy::{app::AppExit, ecs::system::SystemState};
    pub use iyes_loopless::prelude::*;

    // Bracket Lib
    pub use bracket_lib::prelude::Rect;
    pub use bracket_lib::prelude::*;

    // Random Helper Crates
    pub use rayon::prelude::*;

    // Local Helper Libs
    pub use bo_ecs::prelude::*;
    pub use bo_logging::prelude::*;
    pub use bo_map::prelude::*;
    pub use bo_utils::prelude::*;

    pub use crate::spawner;

    pub use crate::effects::*;
    pub use crate::render::*;
    pub use crate::resources::*;
    pub use crate::rng::*;
    pub use crate::setup::*;
    pub use crate::systems::*;
    pub use crate::turn::*;

    pub const SCREEN_WIDTH: usize = 112;
    pub const SCREEN_HEIGHT: usize = 31;

    pub const LAYER_MAP: usize = 0;
    pub const LAYER_DECOR: usize = 1;
    pub const LAYER_ITEMS: usize = 2;
    pub const LAYER_CHARS: usize = 3;
    pub const LAYER_TEXT: usize = 4;
    pub const LAYER_PARTICLES: usize = 5;

    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_DECOR: usize = 1000;
    pub const BATCH_ITEMS: usize = 2000;
    pub const BATCH_CHARS: usize = 3000;
    pub const BATCH_PARTICLES: usize = 4000;
    pub const BATCH_UI: usize = 10_000;
    pub const BATCH_UI_INV: usize = 15_000;
    pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything
}

pub use prelude::*;

embedded_resource!(FONT, "../resources/font.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");
embedded_resource!(GAME_FONT, "../resources/game_font.png");

fn main() -> BError {
    link_resource!(FONT, "resources/font.png");
    link_resource!(FONT, "resources/font.png");
    link_resource!(GAME_FONT, "resources/game_font.png");

    let mut context = BTermBuilder::new()
        .with_title("Secbot - 2022") // Set Window Title
        .with_tile_dimensions(16, 16) // Calculate window size with this...
        .with_dimensions(56, 31) // ..Assuming a console of this size
        .with_fps_cap(60.0) // Limit game speed
        ////////////////////////////////////////////////////////////////////////////////
        .with_font("font.png", 16, 16) // Load big font
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        ////////////////////////////////////////////////////////////////////////////////
        .with_simple_console(56, 31, "font.png") // Console 0: Base map
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 1: Decorations
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 2: Items
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 3: Characters
        .with_sparse_console(112, 31, "vga.png") // Console 4: User Interface
        .with_sparse_console(56, 31, "font.png") // Console 5: Particles
        ////////////////////////////////////////////////////////////////////////////////
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
