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
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        Self { world }
    }

    pub fn new_game(&mut self) {
        let map = Map::new_map_rooms_and_corridors();
        let start_pos = map.starting_point;

        let entity = self.world.spawn().insert(Position::new(start_pos)).id();

        // Resource
        self.world.insert_resource(map);
        self.world.insert_resource(TurnState::PreRun);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {}
}
