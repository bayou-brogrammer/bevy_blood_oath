use super::*;

pub enum DungeonModeResult {
    Done,
}

pub struct DungeonMode {
    pub schedule: Schedule,
    pub consoles: Vec<usize>,
}

fn app_quit_dialog() -> (ModeControl, ModeUpdate) {
    (
        ModeControl::Push(AppQuitDialogMode::new().into()),
        ModeUpdate::Immediate,
    )
}

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
impl DungeonMode {
    pub fn new(world: &mut World) -> Self {
        let schedule = setup_scheduler(world);

        BTerm::cls_all();

        // let mut consoles: Vec<(Box<dyn Console + 'static>, usize, bool)> = vec![
        //     (SimpleConsole::init(56, 31), GAME_FONT, true), // Map Layer - #0
        //     (SparseConsole::init(56, 31), GAME_FONT, false), // Decorations Layer - #1
        //     (SparseConsole::init(56, 31), GAME_FONT, false), // Items Layer - #2
        //     (SparseConsole::init(112, 31), TEXT_FONT, true), // User Interface Layer - #3
        // ];

        // for (console, layer, with_background) in consoles.drain(..) {
        //     match with_background {
        //         true => ctx.register_console(console, layer),
        //         false => ctx.register_console_no_bg(console, layer),
        //     };
        // }

        Self {
            schedule,
            consoles: vec![LAYER_MAP_CHAR, LAYER_DECOR, LAYER_ITEMS, LAYER_TEXT],
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        // Insert input into world
        world.insert_resource(ctx.key);
        // world.insert_resource(Mouse::new(ctx.mouse_point(), ctx.left_click));
        self.schedule.run(world);

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, _active: bool) {
        render::clear_all_consoles(ctx, &self.consoles);
        ctx.set_active_console(0);
        render::render_camera(ctx, world);
    }
}
