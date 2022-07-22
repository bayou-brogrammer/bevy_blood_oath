use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;

use crate::prelude::*;

struct CachedExitEvents<'w> {
    state: SystemState<(Res<'w, StateStack<TurnState>>, Option<Res<'w, AppExit>>)>,
}

pub struct GameWorld {
    pub app: App,
    pub ui_consoles: SmallVec<[usize; 6]>,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        // Add Time Resource to the world
        app.init_resource::<Time>();
        app.add_system(|mut time: ResMut<Time>| time.update());

        app.add_plugin(SetupPlugin);
        app.add_plugin(RenderPlugin);

        let system_state: SystemState<(Res<StateStack<TurnState>>, Option<Res<AppExit>>)> =
            SystemState::new(&mut app.world);

        app.insert_resource(CachedExitEvents { state: system_state });
        app.insert_resource(StateStack::new(TurnState::MainMenu));
        app.insert_resource(ReportExecutionOrderAmbiguities);

        Self {
            app,
            ui_consoles: smallvec![
                LAYER_MAP,
                LAYER_DECOR,
                LAYER_ITEMS,
                LAYER_CHARS,
                LAYER_TEXT,
                LAYER_PARTICLES
            ],
        }
    }

    fn inject_context(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        self.app.insert_resource(ctx.key);
        self.app.insert_resource(ctx.mouse_pos());
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.clear_consoles(self.ui_consoles.as_slice());
        self.inject_context(ctx);

        self.app.world.resource_scope(|world, mut cached_state: Mut<CachedExitEvents>| {
            let (state, exit_event) = cached_state.state.get(world);

            match (state.current(), exit_event) {
                (_, Some(_)) => ctx.quit(),
                (TurnState::Targeting { range, item }, _) => {
                    gui::ranged_targeting(ctx, world, *range, *item)
                }
                _ => {}
            }
        });

        self.app.update();
        render_draw_buffer(ctx).expect("Render error");
    }
}
