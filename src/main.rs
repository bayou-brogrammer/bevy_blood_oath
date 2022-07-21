#![feature(adt_const_params)]
#![allow(incomplete_features)]

pub mod gamelog;
pub mod spawner;

mod modes;
mod resources;
mod rng;

mod prelude {
    pub use bevy::ecs::event::Events;
    pub use bevy::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use bracket_lib::prelude::Rect;
    pub use bracket_lib::prelude::*;

    pub use lazy_static::*;
    pub use rayon::prelude::*;

    pub use bo_ecs::prelude::*;
    pub use bo_map::prelude::*;
    pub use bo_utils::prelude::*;

    pub use crate::gamelog;
    pub use crate::spawner;

    pub use crate::modes::*;
    pub use crate::resources::*;
    pub use crate::rng::*;

    pub const LAYER_MAP: usize = 0;
    pub const LAYER_DECOR: usize = 1;
    pub const LAYER_ITEMS: usize = 2;
    pub const LAYER_CHARS: usize = 3;
    pub const LAYER_TEXT: usize = 4;

    pub const SCREEN_WIDTH: usize = 112;
    pub const SCREEN_HEIGHT: usize = 31;
}

pub use prelude::*;

pub struct GameWorld {
    pub mode_stack: ModeStack,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            mode_stack: ModeStack::new(vec![main_menu::MainMenuMode::new().into()]),
        }
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode_stack.tick(ctx) {
            RunControl::Quit => {
                println!("Run Control Quit");
                ctx.quit();
            }
            RunControl::Update => {}
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

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
        // .with_font("game_tileset.png", 16, 16) // load game font
        .with_font("font.png", 16, 16) // Load big font
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        ////////////////////////////////////////////////////////////////////////////////
        .with_simple_console(56, 31, "font.png") // Console 0: Base map
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 1: Decorations
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 2: Items
        .with_sparse_console_no_bg(56, 31, "font.png") // Console 3: Characters
        .with_sparse_console(112, 31, "vga.png") // Console 4: User Interface
        ////////////////////////////////////////////////////////////////////////////////
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
