use super::*;
use bevy::{
    app::AppExit,
    ecs::{schedule::ReportExecutionOrderAmbiguities, system::SystemState},
};

mod setup;
use setup::*;

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum DungeonModeResult {
    Done,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////
pub struct DungeonMode {
    app: App,
    ui_consoles: SmallVec<[usize; 2]>,
}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DungeonMode").field("consoles", &self.ui_consoles).finish()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct CachedExitEvents<'w> {
    state: SystemState<(Res<'w, TurnState>, Option<Res<'w, AppExit>>)>,
}

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
impl DungeonMode {
    pub fn new() -> Self {
        BTerm::clear_all_internal_consoles();

        let mut app = App::new();

        // Add Time Resource to the world
        app.init_resource::<Time>();
        app.add_system(|mut time: ResMut<Time>| time.update());

        // Setup Scheduler
        setup_events(&mut app);
        setup_stages(&mut app);
        setup_debug_systems(&mut app);

        app.add_plugin(RenderPlugin);
        app.add_plugin(SystemsPlugin);

        DungeonMode::setup_game(&mut app.world);

        Self { app, ui_consoles: smallvec![LAYER_MAP, LAYER_TEXT, LAYER_LOG] }
    }

    pub fn setup_game(world: &mut World) {
        let map = Map::new(0, SCREEN_WIDTH, SCREEN_HEIGHT, "Dungeon");
        let start_pos = map.starting_point;

        // Spawn Player
        spawner::spawn_player(world, start_pos);

        // Spawn Enemies
        map.rooms.iter().skip(1).for_each(|room| {
            spawner::spawn_room(world, room);
        });

        spawner::magic_missile_scroll(world, start_pos);
        spawner::fireball_scroll(world, start_pos);

        let system_state: SystemState<(Res<TurnState>, Option<Res<AppExit>>)> = SystemState::new(world);

        // Resource
        world.insert_resource(map);
        world.insert_resource(start_pos);
        world.insert_resource(ParticleBuilder::new());
        world.insert_resource(ReportExecutionOrderAmbiguities);
        world.insert_resource(camera::GameCamera::new(start_pos));
        world.insert_resource(CachedExitEvents { state: system_state });
        world.insert_resource(TurnState::AwaitingInput);

        bo_logging::Logger::new().append("Welcome to").append_with_color("Rusty Roguelike", CYAN).log();
    }

    fn inject_context(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        self.app.insert_resource(ctx.key);
        self.app.insert_resource(Mouse::new(ctx.mouse_point(), ctx.mouse_pos(), ctx.left_click));
    }

    pub fn tick(&mut self, ctx: &mut BTerm, _pop_result: &Option<ModeResult>) -> ModeControl {
        self.inject_context(ctx);
        self.app.update();

        self.app.world.resource_scope(|world, mut cached_state: Mut<CachedExitEvents>| {
            let (turn_state, exit_event) = cached_state.state.get(world);

            if exit_event.is_some() {
                return ModeControl::Pop(DungeonModeResult::Done.into());
            }

            match *turn_state {
                TurnState::GameOver => ModeControl::Switch(GameOverMode::new().into()),
                _ => ModeControl::Stay,
            }
        })
    }

    pub fn draw(&mut self, ctx: &mut BTerm, _active: bool) {
        ctx.clear_consoles(self.ui_consoles.as_slice());
    }
}
