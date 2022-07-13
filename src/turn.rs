use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NewState {
    NoChange,
    Wait,
    Tick,
    LeftMap,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    Start,
    WaitingForInput,
    Ticking,
    GameOverLeft,
    // Modal(String, String),
}

pub struct State {
    pub world: World,
    pub(crate) input_dispatcher: Box<dyn crate::systems::UnifiedDispatcher + 'static>,
    pub(crate) ticking_dispatcher: Box<dyn crate::systems::UnifiedDispatcher + 'static>,
    pub(crate) gui_dispatcher: Box<dyn crate::systems::UnifiedDispatcher + 'static>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        // Dispatchers
        let mut input_dispatcher = crate::systems::new_input_dispatcher();
        let mut ticking_dispatcher = crate::systems::new_ticking_dispatcher();
        let mut gui_dispatcher = crate::render::new_gui_dispatcher();

        input_dispatcher.setup(&mut world);
        ticking_dispatcher.setup(&mut world);
        gui_dispatcher.setup(&mut world);

        Self::register_systems(&mut world);

        // Resources
        world.insert(TurnState::Start);

        Self {
            world,
            input_dispatcher,
            ticking_dispatcher,
            gui_dispatcher,
        }
    }

    fn register_systems(world: &mut World) {
        // Tags
        world.register::<Player>();
        world.register::<Colonist>();
        world.register::<Door>();

        // Generics
        world.register::<Position>();
        world.register::<Glyph>();
        world.register::<Description>();
        world.register::<ColonistStatus>();

        // Stats
        world.register::<FieldOfView>();
        world.register::<Name>();

        // Activate
        world.register::<TileTrigger>();
    }

    fn run_all_systems(&mut self) {
        self.input_dispatcher.run_now(&mut self.world);
        self.ticking_dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    fn run_input_systems(&mut self) {
        self.input_dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    fn run_ticking_systems(&mut self) {
        self.ticking_dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn new_game(&mut self) {
        let map = Map::new(&mut self.world);

        dbg!("new game");
        let player = self
            .world
            .create_entity()
            .with(Position::with_pt(map.get_current().starting_point, 0))
            .with(Glyph {
                glyph: to_cp437('@'),
                color: ColorPair::new(YELLOW, BLACK),
            })
            .with(Description(
                "Everybody's favorite Bracket Corp SecBot".to_string(),
            ))
            .with(Name("SecBot".to_string()))
            .with(FieldOfView::new(8))
            .build();

        self.world
            .write_storage::<Player>()
            .insert(player, Player { id: player })
            .unwrap();

        self.world.insert(Player { id: player });

        self.world.insert(map);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        render::clear_all_consoles(ctx);

        if let Some(quit) = self.world.get_mut::<Quit>() {
            if quit.0 {
                ctx.quit();
            }
        }

        self.world.insert(Key(ctx.key));
        self.world.insert(Mouse {
            left_click: ctx.left_click,
            mouse_pos: Point::from_tuple(ctx.mouse_pos()),
        });

        let mut current_turn_state;
        {
            let state = self.world.fetch::<TurnState>();
            current_turn_state = *state;
        }

        // Rendering GUI
        match current_turn_state {
            TurnState::Start => {}
            TurnState::GameOverLeft => {}
            _ => {
                render::render(&mut self.world);
                render_draw_buffer(ctx).expect("Render error");
            }
        }

        let next_state = match current_turn_state {
            TurnState::Start => {
                self.new_game();
                self.run_all_systems();
                NewState::Wait
            }
            TurnState::WaitingForInput => {
                self.run_input_systems();
                NewState::NoChange
            }
            TurnState::Ticking => {
                self.run_ticking_systems();
                NewState::Wait
            }
            // TurnState::Modal { title, body } => render::modal(ctx, &title, &body),
            // TurnState::GameOverLeft => render::gameover::game_over_dead(ctx, &mut self.world),
            _ => NewState::NoChange,
        };

        let mut turn_state = self.world.get_mut::<TurnState>().unwrap();
        match next_state {
            NewState::NoChange => {}
            NewState::LeftMap => *turn_state = TurnState::GameOverLeft,
            NewState::Wait => *turn_state = TurnState::WaitingForInput,
            NewState::Tick => *turn_state = TurnState::Ticking,
        }
    }
}
