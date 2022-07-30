#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod spawner;

mod effects;
mod render;
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
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_terminal::prelude::*;

    // Random Helper Crates
    pub use rayon::prelude::*;

    // Local Helper Libs
    pub use bo_ecs::prelude::*;
    pub use bo_logging::prelude::*;
    pub use bo_map::prelude::*;
    pub use bo_pathfinding::prelude::*;
    pub use bo_utils::prelude::*;

    pub use crate::spawner;

    pub use crate::effects::*;
    pub use crate::render::*;
    pub use crate::setup::*;
    pub use crate::systems::*;
    pub use crate::turn::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;

    pub const UI_WIDTH: i32 = 80;
    pub const UI_HEIGHT: i32 = 30;

    pub const LAYER_MAP: usize = 0;
    pub const LAYER_LOG: usize = 1;
    pub const LAYER_TEXT: usize = 2;

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

fn main() -> BError {
    let mut context = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_fps_cap(60.0)
        .with_tile_dimensions(12, 12)
        .with_dimensions(80, 60)
        .with_title("Roguelike Tutorial")
        .with_resource_path("assets/")
        .with_font("vga.png", 8, 16)
        // Log Box #1
        .with_sparse_console(80, 30, "vga.png")
        // UI #2
        .with_sparse_console(80, 30, "vga.png")
        .with_vsync(false)
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
