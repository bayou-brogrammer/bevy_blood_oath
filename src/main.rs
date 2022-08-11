#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod raws;
pub mod rng;
pub mod spawner;

mod actions;
mod ecs;
mod effects;
mod map;
mod modes;
mod random_table;
mod render;
mod rex_assets;
mod utils;

mod prelude {
    // Bevy
    pub use bevy::ecs::event::Events;
    pub use bevy::prelude::*;
    pub use bevy::{app::AppExit, ecs::system::SystemState};
    pub use iyes_loopless::prelude::*;

    // Bracket Lib
    pub use bracket_lib::prelude::*;

    // Random Helper Crates
    pub use lazy_static::lazy_static;
    pub use serde::{Deserialize, Serialize};

    // Local Helper Libs
    pub use bo_logging::*;
    pub use bo_pathfinding::prelude::*;

    // Local Crates
    pub use crate::impl_default;
    pub use crate::impl_new;
    pub use crate::raws;
    pub use crate::rng;
    pub use crate::spawner;

    pub use crate::actions::*;
    pub use crate::ecs::*;
    pub use crate::effects::*;
    pub use crate::map::*;
    pub use crate::modes::*;
    pub use crate::random_table::*;
    pub use crate::raws::*;
    pub use crate::render::*;
    pub use crate::rex_assets::*;
    pub use crate::utils::*;

    pub type BoxedError = Box<dyn std::error::Error>;
    pub use crate::BracketContext;

    pub const MAP_GEN_TIMER: f32 = 100.0;
    pub const SHOW_BOUNDARIES: bool = true;
    pub const SHOW_MAPGEN_VISUALIZER: bool = false;

    pub const SCREEN_WIDTH: i32 = 56;
    pub const SCREEN_HEIGHT: i32 = 31;
    pub const UI_WIDTH: i32 = (SCREEN_WIDTH as f32 * 1.6) as i32;
    pub const UI_HEIGHT: i32 = SCREEN_HEIGHT;

    pub const LAYER_ZERO: usize = 0;
    pub const LAYER_TEXT: usize = 1;

    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_DECOR: usize = 1000;
    pub const BATCH_ITEMS: usize = 2000;
    pub const BATCH_CHARS: usize = 3000;
    pub const BATCH_UI: usize = 10_000;
    pub const BATCH_UI_INV: usize = 15_000;
    pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything
}

pub use prelude::*;

pub struct BracketContext {
    pub mouse_pt: Point,
    pub frame_time_ms: f32,
    pub char_size: (u32, u32),
    pub mouse_pos: (i32, i32),
    pub mouse_left_click: bool,
}

impl_new!(
    BracketContext,
    frame_time_ms: f32,
    char_size: (u32, u32),
    mouse_pos: (i32, i32),
    mouse_pt: Point,
    mouse_left_click: bool
);

pub struct GameWorld {
    pub app: App,
    pub wait_for_event: bool,
    pub mode_stack: ModeStack,
    pub active_mouse_pos: Point,
}

impl_default!(GameWorld);

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        // When building for WASM, print panics to the browser console
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        // Intent Events
        app.add_event::<WantsToMove>();
        app.add_event::<WantsToAttack>();
        app.add_event::<WantsToUseItem>();
        // Item Events
        app.add_event::<WantsToDropItem>();
        app.add_event::<WantsToEquipItem>();
        app.add_event::<WantsToPickupItem>();
        app.add_event::<WantsToRemoveItem>();

        /*
         * We need multiple stages to handle the following:
         * 1. Handle input from player and generate actions
         * 2. Generate Player Actions
         * 3. Handle Player Actions
         * 4. Generate AI Actions
         * 5. Handle AI Actions
         * 6. Effects + Cleanup
         */
        app.add_stage_after(CoreStage::Update, PlayerStage::GenerateActions, SystemStage::parallel())
            .add_stage_after(
                PlayerStage::GenerateActions,
                PlayerStage::HandleActions,
                SystemStage::parallel(),
            )
            .add_stage_after(PlayerStage::HandleActions, PlayerStage::Effects, SystemStage::parallel())
            .add_stage_after(PlayerStage::Effects, PlayerStage::Cleanup, SystemStage::parallel());

        // AI Stages
        app.add_stage_after(PlayerStage::Cleanup, AIStage::HandleAI, SystemStage::parallel())
            .add_stage_after(AIStage::HandleAI, AIStage::GenerateActions, SystemStage::parallel())
            .add_stage_after(AIStage::GenerateActions, AIStage::HandleActions, SystemStage::parallel())
            .add_stage_after(AIStage::HandleActions, AIStage::Effects, SystemStage::parallel())
            .add_stage_after(AIStage::Effects, AIStage::Cleanup, SystemStage::parallel());

        // Add Time Resource to the world
        app.init_resource::<Time>();
        app.add_system(|mut time: ResMut<Time>| time.update());

        app.insert_resource(RexAssets::new());
        app.insert_resource(MenuMemory::new());
        app.add_loopless_state(GameCondition::MainMenu);

        raws::load_raws();

        Self {
            app,
            wait_for_event: false,
            active_mouse_pos: Point::zero(),
            mode_stack: ModeStack::new(vec![main_menu_mode::MainMenuMode::new().into()]),
        }
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
        self.inject_bracket_context(ctx);

        if !self.wait_for_event {
            self.active_mouse_pos = ctx.mouse_point();

            match self.mode_stack.update(ctx, &mut self.app) {
                RunControl::Update => {}
                RunControl::Quit => ctx.quit(),
                RunControl::WaitForEvent => self.wait_for_event = true,
            }
        } else {
            let new_mouse = ctx.mouse_point();

            // Handle Keys & Mouse Clicks
            if ctx.key.is_some() || ctx.left_click {
                self.wait_for_event = false;
            }

            // Handle Mouse Movement
            if new_mouse != self.active_mouse_pos {
                self.wait_for_event = false;
                self.active_mouse_pos = new_mouse;
            }
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

bracket_lib::prelude::add_wasm_support!();

embedded_resource!(VGA_FONT, "../resources/vga.png");
embedded_resource!(TERMINAL_8X8_FONT, "../resources/terminal8x8.png");
embedded_resource!(TERMINAL_10X16_FONT, "../resources/terminal10x16.png");

fn main() -> BError {
    link_resource!(VGA_FONT, "resources/vga.png");
    link_resource!(TERMINAL_8X8_FONT, "resources/terminal8x8.png");
    link_resource!(TERMINAL_10X16_FONT, "resources/terminal10x16.png");

    let mut context = BTermBuilder::new()
        .with_title("Secbot - 2021 7DRL") // Set Window Title
        .with_tile_dimensions(16, 16)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT) // ..Assuming a console of this size
        .with_fps_cap(60.0) // Limit game speed
        .with_font("terminal10x16.png", 10, 16)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        ////////////////////////////////////////////////////////////////////
        // Cosoles
        ////////////////////////////////////////////////////////////////////
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // Map + Char
        .with_sparse_console(UI_WIDTH, UI_HEIGHT, "terminal10x16.png") // UI
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
