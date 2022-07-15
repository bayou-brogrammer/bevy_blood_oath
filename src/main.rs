#![feature(decl_macro)]
#![feature(is_some_with)]

pub mod render;
pub mod spawner;
pub mod systems;

mod components;
mod game;
mod gamelog;
mod map;
mod rng;
mod turn;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use lazy_static::*;

    pub use specs::prelude::World;
    pub use specs::prelude::*;
    pub use specs::Component;

    pub use crate::render;
    pub use crate::spawner;
    pub use crate::systems;

    pub use crate::components::*;
    pub use crate::game::*;
    pub use crate::map::*;
    pub use crate::rng::*;
    pub use crate::systems::*;
    pub use crate::turn::*;

    pub const LAYER_MAP: usize = 0;
    pub const LAYER_DECOR: usize = 1;
    pub const LAYER_ITEMS: usize = 2;
    pub const LAYER_CHR: usize = 3;
    pub const LAYER_TEXT: usize = 4;
}

pub use prelude::*;

embedded_resource!(TILE_FONT, "../resources/terminal8x8.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");

fn main() -> BError {
    link_resource!(TILE_FONT, "resources/terminal8x8.png");
    link_resource!(VGA_FONT, "resources/vga.png");

    let context = BTermBuilder::new()
        .with_title("Secbot - 2022")
        .with_tile_dimensions(16, 16) // Calculate window size with this...
        .with_dimensions(56, 31) // ..Assuming a console of this size
        .with_fps_cap(60.0)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_simple_console(56, 31, "terminal8x8.png") // Console 0: Base map
        .with_sparse_console_no_bg(56, 31, "terminal8x8.png") // Console 1: Decorations
        .with_sparse_console_no_bg(56, 31, "terminal8x8.png") // Console 2: Items
        .with_sparse_console_no_bg(56, 31, "terminal8x8.png") // Console 3: Characters
        .with_sparse_console(112, 31, "vga.png") // Console 4: User Interface
        .build()?;

    let mut state = State::new();
    state.new_game();
    main_loop(context, state)
}
