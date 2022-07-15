use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    pub world: World,
    pub(crate) ticking_dispatcher: Box<dyn crate::systems::UnifiedDispatcher + 'static>,
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
        let mut ticking_dispatcher = crate::systems::new_ticking_dispatcher();

        ticking_dispatcher.setup(&mut world);
        Self::register_systems(&mut world);

        Self {
            world,
            ticking_dispatcher,
        }
    }

    fn register_systems(world: &mut World) {
        // Tags
        world.register::<Player>();
        world.register::<Monster>();
        world.register::<BlocksTile>();

        // Generics
        world.register::<Position>();
        world.register::<Glyph>();
        world.register::<FieldOfView>();
        world.register::<Description>();
        world.register::<Name>();

        // Stats
        world.register::<CombatStats>();
        world.register::<WantsToMelee>();
        world.register::<SufferDamage>();
    }

    fn run_systems(&mut self) {
        self.ticking_dispatcher.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn new_game(&mut self) {
        let map = Map::new_map_rooms_and_corridors();

        let start_pos = map.starting_point;
        let player = self
            .world
            .create_entity()
            .with(Player)
            .with(Position::new(start_pos))
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

        // Resources
        self.world.insert(map);
        self.world.insert(player);
        self.world.insert(start_pos);
        self.world.insert(TurnState::PreRun);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.world.fetch::<TurnState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            TurnState::PreRun => {
                self.run_systems();
                newrunstate = TurnState::AwaitingInput;
            }
            TurnState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            TurnState::PlayerTurn => {
                self.run_systems();
                newrunstate = TurnState::MonsterTurn;
            }
            TurnState::MonsterTurn => {
                self.run_systems();
                newrunstate = TurnState::AwaitingInput;
            }
        }

        {
            let mut runwriter = self.world.write_resource::<TurnState>();
            *runwriter = newrunstate;
        }

        systems::damage_system::delete_the_dead(&mut self.world);

        draw_map(&self.world, ctx);

        let positions = self.world.read_storage::<Position>();
        let glyphs = self.world.read_storage::<Glyph>();
        let map = self.world.fetch::<Map>();

        for (pos, render) in (&positions, &glyphs).join() {
            let idx = map.point2d_to_index(pos.0);
            if map.visible[idx] {
                ctx.set(
                    pos.0.x,
                    pos.0.y,
                    render.color.fg,
                    RGBA::from_u8(0, 0, 0, 0),
                    render.glyph,
                )
            }
        }
    }
}
