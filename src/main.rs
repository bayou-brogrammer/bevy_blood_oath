#![allow(clippy::all)]
#![deny(clippy::correctness)]

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
    pub use crate::impl_new;
    pub use crate::rng;
    pub use crate::spawner;

    pub use crate::actions::*;
    pub use crate::ecs::*;
    pub use crate::effects::*;
    pub use crate::map::*;
    pub use crate::modes::*;
    pub use crate::random_table::*;
    pub use crate::render::*;
    pub use crate::rex_assets::*;
    pub use crate::utils::*;

    pub type BoxedError = Box<dyn std::error::Error>;
    pub use crate::BracketContext;

    pub const MAP_GEN_TIMER: f32 = 100.0;
    pub const SHOW_BOUNDARIES: bool = true;
    pub const SHOW_MAPGEN_VISUALIZER: bool = false;

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
         * 6. Effects
         */
        app.add_stage_after(
            CoreStage::Update,
            GameStage::GeneratePlayerActions,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::GeneratePlayerActions,
            GameStage::HandlePlayerActions,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::HandlePlayerActions,
            GameStage::GenerateAIActions,
            SystemStage::parallel(),
        )
        // AI Stages
        .add_stage_after(
            GameStage::GenerateAIActions,
            GameStage::HandleAIActions,
            SystemStage::parallel(),
        )
        .add_stage_after(GameStage::HandleAIActions, GameStage::AICleanup, SystemStage::parallel())
        .add_stage_after(
            GameStage::HandleAIActions,
            GameStage::Cleanup,
            SystemStage::parallel(),
        );

        // Add Time Resource to the world
        app.init_resource::<Time>();
        app.add_system(|mut time: ResMut<Time>| time.update());

        app.insert_resource(RexAssets::new());
        app.insert_resource(MenuMemory::new());
        app.add_loopless_state(GameCondition::MainMenu);

        // #[cfg(debug_assertions)]
        // app.add_system_set_to_stage(
        //     CoreStage::Update,
        //     ConditionSet::new()
        //         .with_system(|m_q: Query<&Point, Added<Monster>>, i_q: Query<&Point, Added<Item>>| {
        //             for pos in m_q.iter() {
        //                 eprintln!("Monster Spawned at {:?}", pos)
        //             }
        //             for pos in i_q.iter() {
        //                 eprintln!("Item Spawned at {:?}", pos)
        //             }
        //         })
        //         .into(),
        // );

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

        // quit_system(ctx, &mut self.app.world);
        render_draw_buffer(ctx).expect("Render error");
    }
}

embedded_resource!(TERMINAL_FONT, "../resources/terminal8x8.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");

fn main() -> BError {
    link_resource!(TERMINAL_FONT, "resources/terminal8x8.png");
    link_resource!(VGA_FONT, "resources/vga.png");

    let mut context = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_fps_cap(60.0)
        .with_tile_dimensions(12, 12)
        .with_dimensions(80, 60)
        .with_title("Roguelike Tutorial")
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
