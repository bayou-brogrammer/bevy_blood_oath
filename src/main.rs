pub mod gamelog;
pub mod render;
pub mod spawner;

mod bitgrid;
mod components;
mod events;
mod game;
mod map;
mod modes;
mod resources;
mod rng;
mod turn;
mod utils;

mod prelude {
    pub use bevy_ecs::event::Events;
    pub use bevy_ecs::prelude::*;
    pub use bevy_ecs::system::SystemState;

    pub use bracket_lib::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub use lazy_static::*;

    pub use rayon::prelude::*;

    pub use crate::gamelog;
    pub use crate::render;
    pub use crate::spawner;

    pub use crate::bitgrid::*;
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::game::*;
    pub use crate::map::*;
    pub use crate::modes::*;
    pub use crate::render::*;
    pub use crate::resources::*;
    pub use crate::rng::*;
    pub use crate::turn::*;
    pub use crate::utils::*;

    pub const LAYER_MAP_CHAR: usize = 0;
    pub const LAYER_DECOR: usize = 1;
    pub const LAYER_ITEMS: usize = 2;
    pub const LAYER_TEXT: usize = 3;
    pub const LAYER_MAIN_MENU: usize = 4;

    pub const SCREEN_WIDTH: usize = 112;
    pub const SCREEN_HEIGHT: usize = 31;
}

pub use prelude::*;

embedded_resource!(GAME_FONT, "../resources/game_tileset.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");

fn main() -> BError {
    link_resource!(GAME_FONT, "resources/game_tileset.png");
    link_resource!(VGA_FONT, "resources/vga.png");

    let mut context = BTermBuilder::new()
        .with_title("Secbot - 2021 7DRL") // Set Window Title
        .with_tile_dimensions(16, 16) // Calculate window size with this...
        .with_dimensions(56, 31) // ..Assuming a console of this size
        .with_fps_cap(60.0) // Limit game speed
        .with_font("game_tileset.png", 16, 16) // load game font
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_font("font.png", 16, 16) // Load big font
        ////////////////////////////////////////////////////////////////////////////////
        // Map + Entities Layer - #0
        .with_simple_console(56, 31, "font.png")
        // Decorations Layer - #1
        .with_sparse_console_no_bg(56, 31, "font.png")
        // Items Layer - #2
        .with_sparse_console_no_bg(56, 31, "font.png")
        // UI Layer - #3
        .with_sparse_console(112, 31, "vga.png")
        // Main Menu Layer - #4
        .with_sparse_console(80, 31, "vga.png")
        ////////////////////////////////////////////////////////////////////////////////
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
