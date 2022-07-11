mod components;
mod game;
mod map;
mod resources;
mod rng;
mod text;

pub mod render;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use lazy_static::*;

    pub use legion::storage::PackOptions;
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
    ecs: World,
    map: map::Map,
    turn: TurnState,
    resources: Resources,
    scheduler: Schedule,
}

impl State {
    fn new() -> Self {
        let options = WorldOptions {
            groups: vec![<(Position, Glyph)>::to_group()],
        };
        let mut ecs = World::new(options);
        let mut resources = Resources::default();
        let map = map::Map::new(&mut ecs);

        resources.insert(map.clone());
        resources.insert(TurnState::WaitingForInput);
        ecs.pack(PackOptions::force());

        let mut state = Self {
            ecs,
            map,
            resources,
            scheduler: Self::build_scheduler(),
            turn: TurnState::Modal {
                title: "SecBot Has Landed".to_string(),
                body: text::INTRO.to_string(),
            },
        };

        state.new_game();
        state
    }

    fn new_game(&mut self) {
        // Spawn the player
        self.ecs.push((
            Player {},
            Position::with_pt(self.map.get_current().starting_point, 0),
            Glyph {
                glyph: to_cp437('@'),
                color: ColorPair::new(YELLOW, BLACK),
            },
            Description("Everybody's favorite Bracket Corp SecBot".to_string()),
            FieldOfView::new(8),
        ));
    }

    pub fn build_scheduler() -> Schedule {
        Schedule::builder()
            .add_system(systems::fov::fov_system())
            .flush()
            .build()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.scheduler.execute(&mut self.ecs, &mut self.resources);
        let mut map = self.resources.get_mut::<Map>().unwrap();

        render::render_ui_skeleton(ctx);
        map.render(ctx);
        render::render_glyphs(ctx, &self.ecs, &map);

        let mut run_state = self.resources.get_mut::<TurnState>().unwrap();
        let new_state = match &*run_state {
            TurnState::GameOverLeft => render::game_over_left(ctx),
            TurnState::Modal { title, body } => render::modal(ctx, title, body),
            TurnState::WaitingForInput => game::player_turn(ctx, &mut self.ecs, &mut map),
            TurnState::EnemyTurn => NewState::Wait,
            TurnState::PlayerTurn => NewState::Wait,
            _ => NewState::NoChange,
        };

        // let mut turn_state = self.resources.get_mut::<TurnState>().unwrap();
        *run_state = match new_state {
            NewState::Player => TurnState::EnemyTurn,
            NewState::Wait => TurnState::WaitingForInput,
            NewState::Enemy => TurnState::EnemyTurn,
            NewState::LeftMap => TurnState::GameOverLeft,
            NewState::NoChange => run_state.clone(),
        };
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(112, 62)?
        .with_title("Secbot - 2021 7DRL")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
