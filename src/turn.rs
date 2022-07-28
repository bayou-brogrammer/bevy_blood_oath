use crate::prelude::*;

pub struct GameWorld {
    pub app: App,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_loopless_state(GameCondition::MainMenu);
        app.insert_resource(StateStack::new(TurnState::AwaitingInput));

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

        // Handle State Adjustments
        app.add_system_to_stage(CoreStage::PreUpdate, adjust_state.exclusive_system());

        Self { app }
    }

    fn inject_mouse(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(LAYER_MAP);
        self.app.insert_resource(MousePosition::new(ctx.mouse_point(), ctx.mouse_pos()));

        if ctx.left_click {
            self.app.insert_resource(MouseLeftClick(ctx.left_click));
        } else {
            self.app.world.remove_resource::<MouseLeftClick>();
        }
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.clear_consoles(&[LAYER_MAP, LAYER_LOG, LAYER_TEXT]);

        if let Some(key) = ctx.key {
            self.app.insert_resource(key);
        } else {
            self.app.world.remove_resource::<VirtualKeyCode>();
        }

        if let Some(state) = self.app.world.get_resource::<StateStack<TurnState>>() {
            match *state.current() {
                TurnState::Confirm(_) => {}
                _ => self.inject_mouse(ctx),
            }
        }

        self.app.update();

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn adjust_state(world: &mut World) {
    if let Some(set_state) = world.remove_resource::<SetState>() {
        let mut stack = world.resource_mut::<StateStack<TurnState>>();
        stack.set(set_state.0.clone()).unwrap();
    }

    if let Some(_) = world.remove_resource::<PopState>() {
        let mut stack = world.resource_mut::<StateStack<TurnState>>();
        stack.pop().unwrap();
    }
}
