#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod raws;
pub mod rng;
pub mod spawner;

mod actions;
mod ecs;
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
    pub use bracket_state_machine::prelude::*;

    // Random Helper Crates
    pub use lazy_static::lazy_static;
    pub use serde::{Deserialize, Serialize};

    // Local Helper Libs
    pub use bo_logging::*;

    // Local Crates
    pub use crate::impl_default;
    pub use crate::impl_new;
    pub use crate::raws;
    pub use crate::rng;
    pub use crate::spawner;

    pub use crate::actions::*;
    pub use crate::ecs::*;
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

    pub const UI_WIDTH: i32 = (SCREEN_WIDTH as f32 * 2.) as i32;
    pub const UI_HEIGHT: i32 = SCREEN_HEIGHT;
    pub const LOG_DISPLAY_WIDTH: i32 = (SCREEN_WIDTH as f32 * 2.) as i32;

    pub const LAYER_ZERO: usize = 0;
    pub const LAYER_CHAR: usize = 1;
    pub const LAYER_PARTICLE: usize = 2;
    pub const LAYER_TEXT: usize = 3;
    pub const LAYER_LOG: usize = 4;

    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_CHARS: usize = 3000;
    pub const BATCH_UI: usize = 10_000;
    pub const BATCH_UI_INV: usize = 15_000;
    pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything
}

use modes::map_gen::MapGenPlugin;
pub use prelude::*;

use crate::main_menu_mode::MainMenuMode;

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
    pub render_schedule: Schedule,
}

impl_default!(GameWorld);

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        raws::load_raws();

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
        app.add_loopless_state(AppState::MainMenu);

        app.add_plugin(MapGenPlugin);

        // Create a render schedule and a stage
        let mut render_schedule = Schedule::default();
        let mut update = SystemStage::parallel();

        update.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<CameraView>()
                .with_system(ecs::render::map_renderer::map_render)
                .with_system(ecs::render::entity_renderer::entity_render)
                .with_system(ecs::render::entity_renderer::particle_render)
                // .with_system(render::tooltips::render_tooltips)
                .into(),
        );

        render_schedule.add_stage(CoreStage::Update, update);

        // app.add_system(save_scene_system.exclusive_system()).register_type::<Player>();

        Self { app, render_schedule }
    }

    pub fn global_tick(ctx: &mut BTerm, state: &mut GameWorld) {
        ctx.set_active_console(LAYER_ZERO);

        state.app.insert_resource(ctx.key);
        state.app.insert_resource(BracketContext::new(
            ctx.frame_time_ms,
            ctx.get_char_size(),
            ctx.mouse_pos(),
            ctx.mouse_point(),
            ctx.left_click,
        ));
    }
}

bracket_lib::prelude::add_wasm_support!();

embedded_resource!(VGA_FONT, "../resources/vga.png");
embedded_resource!(TERMINAL_8X8_FONT, "../resources/terminal8x8.png");
embedded_resource!(TERMINAL_10X16_FONT, "../resources/terminal10x16.png");

fn main() -> BError {
    env_logger::init();

    link_resource!(VGA_FONT, "resources/vga.png");
    link_resource!(TERMINAL_8X8_FONT, "resources/terminal8x8.png");
    link_resource!(TERMINAL_10X16_FONT, "resources/terminal10x16.png");

    let mut context = BTermBuilder::new()
        .with_title("Secbot - 2021 7DRL") // Set Window Title
        .with_tile_dimensions(16, 16)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT) // ..Assuming a console of this size
        .with_fps_cap(60.0) // Limit game speed
        .with_font("terminal8x8.png", 8, 8)
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_font("terminal10x16.png", 10, 16) // Load easy-to-read font
        .with_font("urizen12x12.png", 13, 13) // Load easy-to-read font
        ////////////////////////////////////////////////////////////////////
        // Cosoles
        ////////////////////////////////////////////////////////////////////
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // Map
        .with_sparse_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // Char
        .with_sparse_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // Particle
        .with_sparse_console(UI_WIDTH, UI_HEIGHT, "vga.png") // UI
        .with_sparse_console(LOG_DISPLAY_WIDTH, UI_HEIGHT, "vga.png") // LOG
        .build()?;

    context.with_post_scanlines(true);

    let mut machine = StateMachine::new(GameWorld::new(), MainMenuMode::new());
    machine.add_global_tick_fn(GameWorld::global_tick);
    main_loop(context, machine)
}

// #[derive(Component, Reflect, Default)]
// #[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
// struct ComponentA {
//     pub x: f32,
//     pub y: f32,
// }

// fn save_scene_system(world: &mut World) {
//     // The TypeRegistry resource contains information about all registered types (including
//     // components). This is used to construct scenes.
//     let type_registry = world.resource::<TypeRegistry>();
//     let scene = DynamicScene::from_world(world, type_registry);

//     // Scenes can be serialized like this:
//     let serialized_scene = scene.serialize_ron(type_registry).unwrap();

//     // Showing the scene in the console
//     println!("{}", serialized_scene);
// }
