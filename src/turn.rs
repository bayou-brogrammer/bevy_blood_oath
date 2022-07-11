use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NewState {
    NoChange,
    Wait,
    Tick,
    LeftMap,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TurnState {
    Start,
    WaitingForInput,
    Ticking,
    GameOverLeft,
    Modal { title: String, body: String },
}

pub struct State {
    pub world: World,
    pub resources: Resources,
    pub input_scheduler: Schedule,
    pub player_scheduler: Schedule,
    pub ai_scheduler: Schedule,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let options = WorldOptions {
            groups: vec![
                <(Position, Glyph)>::to_group(),
                <(Description, Name)>::to_group(),
            ],
        };
        let mut world = World::new(options);
        world.pack(PackOptions::force());

        let mut resources = Resources::default();
        let map = Map::new(&mut world);

        resources.insert(map);
        resources.insert(TurnState::Start);

        Self {
            world,
            resources,
            input_scheduler: player::build_input_scheduler(),
            player_scheduler: player::build_player_scheduler(),
            ai_scheduler: build_ai_scheduler(),
        }
    }

    pub fn new_game(&mut self) {
        let map = self.resources.get::<Map>().unwrap();

        // Spawn the player
        self.world.push((
            Player {},
            Position::with_pt(map.get_current().starting_point, 0),
            Glyph {
                glyph: to_cp437('@'),
                color: ColorPair::new(YELLOW, BLACK),
            },
            Description("Everybody's favorite Bracket Corp SecBot".to_string()),
            Name("SecBot".to_string()),
            FieldOfView::new(8),
        ));
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        render::clear_all_consoles(ctx);
        self.resources.insert(ctx.key);
        self.resources.insert(Mouse {
            left_click: ctx.left_click,
            mouse_pos: Point::from_tuple(ctx.mouse_pos()),
        });

        let current_state = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::GameOverLeft => {}
            TurnState::Start => {}
            _ => {
                render::render_gui(&mut self.world, &mut self.resources);
                render_draw_buffer(ctx).expect("Render error");
            }
        }

        let next_state = match current_state {
            TurnState::Start => {
                self.new_game();

                Schedule::builder()
                    .add_system(player::update_fov_system())
                    .build()
                    .execute(&mut self.world, &mut self.resources);

                *self.resources.get_mut::<TurnState>().unwrap() = TurnState::Modal {
                    title: "Welcome to Bracket Corp".to_string(),
                    body: "Press any key to start".to_string(),
                };

                NewState::NoChange
            }
            TurnState::Modal { title, body } => render::modal(ctx, &title, &body),
            TurnState::GameOverLeft => render::gameover::game_over_dead(ctx, &mut self.world),
            TurnState::WaitingForInput => {
                self.input_scheduler
                    .execute(&mut self.world, &mut self.resources);
                NewState::NoChange
            }
            TurnState::Ticking => {
                self.player_scheduler
                    .execute(&mut self.world, &mut self.resources);

                self.ai_scheduler
                    .execute(&mut self.world, &mut self.resources);

                match *self.resources.get::<TurnState>().unwrap() {
                    TurnState::GameOverLeft => NewState::LeftMap,
                    _ => NewState::Wait,
                }
            }
        };

        let mut turn_state = self.resources.get_mut::<TurnState>().unwrap();
        match next_state {
            NewState::NoChange => {}
            NewState::LeftMap => *turn_state = TurnState::GameOverLeft,
            NewState::Wait => *turn_state = TurnState::WaitingForInput,
            NewState::Tick => *turn_state = TurnState::Ticking,
        }
    }
}
