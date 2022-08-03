use crate::prelude::*;

mod builders;
mod common;
mod maps;
mod random;

pub use builders::*;
pub use common::*;
pub use maps::*;
pub use random::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct MapGenResource {
    pub mapgen_index: usize,
    pub mapgen_timer: f32,
    pub mapgen_history: Vec<Map>,
    pub mapgen_next_state: Option<GameCondition>,
}

impl MapGenResource {
    fn new() -> Self {
        Self {
            mapgen_index: 0,
            mapgen_timer: 0.0,
            mapgen_next_state: Some(GameCondition::InGame),
            mapgen_history: Vec::new(),
        }
    }
}

pub struct MapGenPlugin;
impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(
            GameCondition::MapGen(MapGenState::NewGame),
            setup_new_game.exclusive_system(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameCondition::MapGen(MapGenState::Generate))
                .with_system(run_map_gen)
                .into(),
        );
        // app.add_system(system);
    }
}

// Setup Game for Map Generation
fn setup_new_game(world: &mut World) {
    let mut map_gen = MapGenResource::new();

    let mut builder = map_builders::random_builder(1, 80, 50);
    builder.build_map();

    {
        spawner::spawn_player(world, builder.build_data.starting_position.unwrap());
        builder.spawn_entities(world);
    }

    map_gen.mapgen_history = builder.build_data.history;

    world.insert_resource(map_gen);
    world.insert_resource(builder.build_data.map);
    world.insert_resource(NextState(GameCondition::MapGen(MapGenState::Generate)));
}

fn run_map_gen(mut commands: Commands, mut map_gen: ResMut<MapGenResource>, ctx: Res<BracketContext>) {
    if !SHOW_MAPGEN_VISUALIZER {
        commands.insert_resource(NextState(map_gen.mapgen_next_state.unwrap()))
    } else {
        map_gen.mapgen_timer += ctx.frame_time_ms;

        if map_gen.mapgen_timer > 200.0 {
            map_gen.mapgen_timer = 0.0;
            map_gen.mapgen_index += 1;
            if map_gen.mapgen_index >= map_gen.mapgen_history.len() {
                commands.insert_resource(NextState(map_gen.mapgen_next_state.unwrap()))
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Map Builder Traits
////////////////////////////////////////////////////////////////////////////////

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

////////////////////////////////////////////////////////////////////////////////

pub struct BuilderMap {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub history: Vec<Map>,
    pub spawn_list: Vec<(usize, String)>,
    pub rooms: Option<Vec<Rect>>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub starting_position: Option<Point>,
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            snapshot.revealed.apply_all_bits();
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    pub build_data: BuilderMap,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    starter: Option<Box<dyn InitialMapBuilder>>,
}

impl BuilderChain {
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                width,
                height,
                rooms: None,
                corridors: None,
                history: Vec::new(),
                spawn_list: Vec::new(),
                starting_position: None,
                map: Map::new(new_depth, width, height, name),
            },
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(&mut self.build_data);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.build_data);
        }
    }

    pub fn spawn_entities(&mut self, world: &mut World) {
        for entity in self.build_data.spawn_list.iter() {
            spawner::spawn_entity(world, &(&entity.0, &entity.1));
        }
    }
}

pub fn level_builder(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    console::log(format!("Depth: {}", new_depth));
    match new_depth {
        _ => random_builder(new_depth, width, height),
    }
}
