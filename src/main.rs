pub mod gamelog;
pub mod render;
pub mod spawner;

mod components;
mod events;
mod game;
mod map;
mod resources;
mod rng;
mod turn;

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

    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::game::*;
    pub use crate::map::*;
    pub use crate::render::*;
    pub use crate::resources::*;
    pub use crate::rng::*;
    pub use crate::turn::*;

    pub const LAYERS: usize = 4;
    pub const LAYER_MAP_CHAR: usize = 0;
    pub const LAYER_DECOR: usize = 1;
    pub const LAYER_ITEMS: usize = 2;
    pub const LAYER_TEXT: usize = 3;

    pub const SCREEN_WIDTH: usize = 56;
    pub const SCREEN_HEIGHT: usize = 31;
}

pub use prelude::*;

embedded_resource!(TILE_FONT, "../resources/terminal8x8.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/terminal8x8.png");
    link_resource!(VGA_FONT, "resources/vga.png");

    let mut context = BTermBuilder::new()
        .with_title("Secbot - 2022")
        .with_tile_dimensions(16, 16) // Calculate window size with this...
        .with_dimensions(56, 31) // ..Assuming a console of this size
        .with_fps_cap(60.0)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_simple_console(56, 31, "terminal8x8.png") // Console 0: Base map + Entities
        .with_sparse_console_no_bg(56, 31, "terminal8x8.png") // Console 1: Decorations
        .with_sparse_console_no_bg(56, 31, "terminal8x8.png") // Console 2: Items
        .with_sparse_console(112, 31, "vga.png") // Console 3: User Interface
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
