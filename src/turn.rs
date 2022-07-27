use crate::prelude::*;

pub struct GameWorld {
    pub app: App,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_loopless_state(GameCondition::MainMenu);
        app.add_state(TurnState::AwaitingInput);

        // Add Time Resource to the world
        app.init_resource::<Time>();
        app.add_system(|mut time: ResMut<Time>| time.update());

        // Setup Scheduler
        setup_events(&mut app);
        setup_stages(&mut app);
        setup_debug_systems(&mut app);

        // Plugins
        app.add_plugin(SetupPlugin);
        app.add_plugin(RenderPlugin);
        app.add_plugin(SystemsPlugin);

        Self { app }
    }

    fn inject_context(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        self.app.insert_resource(ctx.key);
        self.app.insert_resource(Mouse::new(ctx.mouse_point(), ctx.mouse_pos(), ctx.left_click));
    }
}

impl bracket_lib::prelude::GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.clear_consoles(&[LAYER_MAP, LAYER_LOG, LAYER_TEXT]);

        self.inject_context(ctx);
        self.app.update();

        render_draw_buffer(ctx).expect("Render error");
    }
}
