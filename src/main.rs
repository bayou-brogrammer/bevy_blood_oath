#![allow(clippy::all)]
#![deny(clippy::correctness)]

pub mod rng;
pub mod spawner;

mod actions;
mod ecs;
mod effects;
mod map;
mod menu_memory;
mod random_table;
mod render;
mod rex_assets;
mod setup;
mod utils;

mod prelude {
    // Bevy
    pub use bevy::ecs::event::Events;
    pub use bevy::prelude::*;
    pub use bevy::{app::AppExit, ecs::system::SystemState};
    pub use iyes_loopless::prelude::*;

    // Bracket Lib
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_terminal::prelude::*;

    // Random Helper Crates
    pub use lazy_static::lazy_static;
    pub use rayon::prelude::*;
    pub use serde::{Deserialize, Serialize};

    // Local Helper Libs
    pub use bo_logging::*;
    pub use bo_pathfinding::prelude::*;

    // Local Crates
    pub use crate::impl_new;
    pub use crate::rng;
    pub use crate::spawner;

    pub use crate::actions::*;
    pub use crate::ecs::*;
    pub use crate::effects::*;
    pub use crate::map::*;
    pub use crate::menu_memory::*;
    pub use crate::random_table::*;
    pub use crate::render::*;
    pub use crate::rex_assets::*;
    pub use crate::setup::*;
    pub use crate::utils::*;

    pub const SHOW_BOUNDARIES: bool = true;
    pub const SHOW_MAPGEN_VISUALIZER: bool = false;
    pub const MAP_GEN_TIMER: f32 = 100.0;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;

    pub const UI_WIDTH: i32 = 80;
    pub const UI_HEIGHT: i32 = 30;

    pub const LAYER_ZERO: usize = 0;
    pub const LAYER_ENTITY: usize = 1;
    pub const LAYER_TEXT: usize = 2;
    pub const LAYER_TOOL: usize = 3;

    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_DECOR: usize = 1000;
    pub const BATCH_ITEMS: usize = 2000;
    pub const BATCH_CHARS: usize = 3000;
    pub const BATCH_UI: usize = 10_000;
    pub const BATCH_UI_INV: usize = 15_000;
    pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything
}

pub use prelude::*;

#[derive(Default)]
pub struct GameWorld {
    pub app: App,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        // Add Time Resource to the world
        app.init_resource::<Time>();
        app.add_system(|mut time: ResMut<Time>| time.update());

        app.insert_resource(RexAssets::new());
        app.insert_resource(MenuMemory::new());
        app.add_loopless_state(GameCondition::MainMenu);

        // Setup Scheduler
        setup_events(&mut app);
        setup_stages(&mut app);
        setup_debug_systems(&mut app);

        // Plugins
        app.add_plugin(SetupPlugin);
        app.add_plugin(map_builders::MapGenPlugin);
        app.add_plugin(RenderPlugin);
        app.add_plugin(SystemsPlugin);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        Self { app }
    }

    fn inject_bracket_context(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_ZERO);

        if let Some(key) = ctx.key {
            self.app.insert_resource(key);
        } else {
            // In order to keep consistency with the Legion version, we need to access Bevy's World
            // directly, since App doesn't support removing resources.
            self.app.world.remove_resource::<VirtualKeyCode>();
        }

        self.app.insert_resource(BracketContext::new(
            ctx.frame_time_ms,
            ctx.get_char_size(),
            ctx.mouse_pos(),
            ctx.mouse_point(),
            ctx.left_click,
        ));
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.clear_consoles(&[LAYER_ZERO, LAYER_ENTITY, LAYER_TEXT]);

        self.inject_bracket_context(ctx);
        self.app.update();

        quit_system(ctx, &mut self.app.world);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn quit_system(ctx: &mut BTerm, world: &mut World) {
    if world.get_resource::<AppExit>().is_some() {
        ctx.quit()
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_fps_cap(60.0)
        .with_tile_dimensions(12, 12)
        .with_dimensions(80, 60)
        .with_title("Roguelike Tutorial")
        .with_resource_path("assets/")
        .with_font("vga.png", 8, 16)
        // Entity Console #1
        .with_sparse_console(80, 60, "terminal8x8.png")
        // TEXT Console #2
        .with_sparse_console(80, 30, "vga.png")
        // ToolTip Console #3
        .with_sparse_console(80, 60, "vga.png")
        .with_vsync(false)
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
