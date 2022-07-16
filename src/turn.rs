use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    AITurn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    Update,
    PlayerStage,
    GenerateAIMoves,
    AIStage,
}

pub struct GameWorld {
    pub world: World,
    pub schedule: Schedule,
    pub player_entity: Entity,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl GameWorld {
    pub fn new() -> Self {
        let mut world = World::new();

        let player_entity = Self::setup_game(&mut world);
        let schedule = setup_scheduler(&mut world);

        Self {
            world,
            schedule,
            player_entity,
        }
    }

    pub fn setup_game(world: &mut World) -> Entity {
        let map = Map::new_map_rooms_and_corridors();
        let start_pos = map.starting_point;

        // Spawn Player
        let player = spawner::spawn_player(world, start_pos);

        // Spawn Enemies
        map.rooms.iter().skip(1).for_each(|room| {
            spawner::spawn_room(world, room);
        });

        // Resource
        world.insert_resource(map);
        world.insert_resource(TurnState::AwaitingInput);

        crate::gamelog::Logger::new()
            .append("Welcome to")
            .append_with_color("Rusty Roguelike", CYAN)
            .log();

        player
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        render::clear_all_consoles(ctx);
        self.world.insert_resource(ctx.key);
        self.world
            .insert_resource(Mouse::new(ctx.mouse_point(), ctx.left_click));

        render::render_camera(self.player_entity, &mut self.world);
        render_draw_buffer(ctx).expect("Render error");

        self.schedule.run(&mut self.world);
    }
}
