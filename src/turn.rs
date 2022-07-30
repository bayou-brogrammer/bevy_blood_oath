use crate::prelude::*;

#[derive(Default)]
pub struct GameWorld {
    pub app: App,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_loopless_state(GameCondition::MainMenu);

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

    fn inject_bracket_context(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        self.app.insert_resource(MousePosition::new(ctx.mouse_point(), ctx.mouse_pos()));

        if let Some(key) = ctx.key {
            self.app.insert_resource(key);
        } else {
            // In order to keep consistency with the Legion version, we need to access Bevy's World
            // directly, since App doesn't support removing resources.
            self.app.world.remove_resource::<VirtualKeyCode>();
        }

        if ctx.left_click {
            self.app.insert_resource(MouseLeftClick(ctx.left_click));
        } else {
            self.app.world.remove_resource::<MouseLeftClick>();
        }
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.clear_consoles(&[LAYER_MAP, LAYER_ENTITY, LAYER_LOG, LAYER_TEXT]);

        self.inject_bracket_context(ctx);
        self.app.update();

        render_draw_buffer(ctx).expect("Render error");
    }
}
