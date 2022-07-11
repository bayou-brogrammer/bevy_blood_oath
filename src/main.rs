mod components;
mod game;
mod map;
mod resources;
mod rng;
mod text;

pub mod render;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use lazy_static::*;

    pub use legion::storage::PackOptions;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::*;
    pub use legion::*;

    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::resources::*;
    pub use crate::rng::*;

    pub use crate::render;
}

pub use prelude::*;

struct State {
    world: World,
    resources: Resources,
    input_scheduler: Schedule,
    player_scheduler: Schedule,
    ai_scheduler: Schedule,
    render_scheduler: Schedule,
}

impl State {
    fn new() -> Self {
        let options = WorldOptions {
            groups: vec![
                <(Position, Glyph)>::to_group(),
                <(Description, Name)>::to_group(),
            ],
        };
        let mut world = World::new(options);
        world.pack(PackOptions::force());

        let mut resources = Resources::default();
        let map = map::Map::new(&mut world);

        resources.insert(map);
        resources.insert(TurnState::Modal {
            title: "SecBot Has Landed".to_string(),
            body: text::INTRO.to_string(),
        });
        resources.insert(NewState::NoChange);

        Self {
            world,
            resources,
            input_scheduler: game::player::build_input_scheduler(),
            render_scheduler: render::build_render_scheduler(),
            player_scheduler: game::player::build_player_scheduler(),
            ai_scheduler: game::build_ai_scheduler(),
        }
    }

    fn new_game(&mut self) {
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
        ctx.cls();
        ctx.set_active_console(0);
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::GameOverLeft => {}
            _ => {
                self.render_scheduler
                    .execute(&mut self.world, &mut self.resources);
            }
        }

        let next_state = match current_state {
            TurnState::Modal { title, body } => render::modal(ctx, &title, &body),
            TurnState::GameOverLeft => render::game_over_left(ctx),
            TurnState::WaitingForInput => {
                self.input_scheduler
                    .execute(&mut self.world, &mut self.resources);

                NewState::NoChange
            }
            // TurnState::Ticking => {
            //     self.player_scheduler
            //         .execute(&mut self.world, &mut self.resources);

            //     // self.enemy_scheduler
            //     //     .execute(&mut self.world, &mut self.resources);

            // match *self.resources.get::<TurnState>().unwrap() {
            //     TurnState::GameOverLeft => NewState::LeftMap,
            //     _ => NewState::Wait,
            // }
            // }
            TurnState::PlayerTurn => {
                self.player_scheduler
                    .execute(&mut self.world, &mut self.resources);

                match *self.resources.get::<TurnState>().unwrap() {
                    TurnState::GameOverLeft => NewState::LeftMap,
                    _ => NewState::Enemy,
                }
            }
            TurnState::EnemyTurn => {
                self.ai_scheduler
                    .execute(&mut self.world, &mut self.resources);

                NewState::Wait
            }
        };

        let mut turn_state = self.resources.get_mut::<TurnState>().unwrap();
        match next_state {
            NewState::NoChange => {}
            NewState::LeftMap => *turn_state = TurnState::GameOverLeft,
            NewState::Wait => *turn_state = TurnState::WaitingForInput,
            NewState::Player => *turn_state = TurnState::PlayerTurn,
            NewState::Enemy => *turn_state = TurnState::EnemyTurn,
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(112, 62)?
        .with_title("Secbot - 2021 7DRL")
        .with_fps_cap(30.0)
        .build()?;

    let mut state = State::new();
    state.new_game();
    main_loop(context, state)
}
