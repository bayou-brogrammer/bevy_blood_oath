use crate::prelude::*;
use bevy::ecs::system::CommandQueue;

fn setup_new_game(world: &mut World) {
    world.clear_entities();

    world.insert_resource(ParticleBuilder::new());
    world.insert_resource(MasterDungeonMap::new());
    world.insert_resource(Map::new(0, 64, 64, "Dummy Map"));

    transition_to_new_map(world, 1);

    bo_logging::clear_log();
    bo_logging::clear_events();
    bo_logging::Logger::new().append("Welcome to").color(CYAN).append("Rusty Roguelike").log();
}

fn goto_level(world: &mut World) {
    freeze_level_entities(world);

    let state = world.resource::<CurrentState<AppState>>();
    let offset = if state.0 == AppState::NextLevel { 1 } else { -1 };
    let new_depth = world.resource::<Map>().depth + offset;

    let dungeon_map = world.resource::<MasterDungeonMap>();
    if dungeon_map.get_map(new_depth).is_some() {
        transition_to_existing_map(world, new_depth, offset);
        thaw_level_entities(world);
    } else {
        transition_to_new_map(world, new_depth)
    }

    // Notify the player
    bo_logging::Logger::new().append("You change level.").log();
}

//////////////////////////////////////////////////////////////////////////////
// Generation
//////////////////////////////////////////////////////////////////////////////

pub fn freeze_level_entities(world: &mut World) {
    // Obtain ECS access
    let mut positions = world.query::<(Entity, &Point)>();
    let map_depth = world.resource::<Map>().depth;
    let player_entity = world.resource::<Entity>();

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);

    // Find positions and make OtherLevelPosition
    for (entity, pos) in positions.iter(world) {
        if entity != *player_entity {
            commands.entity(entity).remove::<Point>();
            commands.entity(entity).insert(OtherLevelPosition::new(*pos, map_depth));
        }
    }

    queue.apply(world);
}

pub fn thaw_level_entities(world: &mut World) {
    // Obtain ECS access
    let mut other_positions = world.query::<(Entity, &OtherLevelPosition)>();
    let map_depth = world.resource::<Map>().depth;
    let player_entity = world.resource::<Entity>();

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, world);

    // Find OtherLevelPosition
    for (entity, pos) in other_positions
        .iter(world)
        .filter(|(entity, pos)| *entity != *player_entity && pos.depth == map_depth)
    {
        commands.entity(entity).insert(pos.pt);
        commands.entity(entity).remove::<OtherLevelPosition>();
    }

    queue.apply(world);
}

fn transition_to_new_map(world: &mut World, new_depth: i32) {
    let mut builder = map_builders::level_builder(new_depth, 80, 50);
    builder.build_map();
    world.insert_resource(builder.build_data.clone());

    // Add Up Stairs
    if new_depth > 1 {
        if let Some(pos) = &builder.build_data.starting_position {
            let up_idx = builder.build_data.map.point2d_to_index(*pos);
            builder.build_data.map.tiles[up_idx] = GameTile::stairs_up();
        }
    }

    let player_start;
    {
        let mut worldmap_resource = world.resource_mut::<Map>();
        *worldmap_resource = builder.build_data.map.clone();
        player_start = builder.build_data.starting_position.unwrap();
    }

    {
        let mut q = world.query_filtered::<(&mut Point, &mut FieldOfView), With<Player>>();
        for (mut pos, mut fov) in q.iter_mut(world) {
            *pos = player_start;
            fov.is_dirty = true;
        }

        if let Some(mut pt) = world.get_resource_mut::<Point>() {
            *pt = player_start;
        }

        if let Some(mut camera) = world.get_resource_mut::<CameraView>() {
            camera.on_player_move(player_start);
        }
    }

    // Store the newly minted map
    let mut dungeon_master = world.resource_mut::<MasterDungeonMap>();
    dungeon_master.store_map(&builder.build_data.map);
}

fn transition_to_existing_map(ecs: &mut World, new_depth: i32, offset: i32) {
    let dungeon_master = ecs.resource::<MasterDungeonMap>();
    let map = dungeon_master.get_map(new_depth).unwrap();
    let player = *ecs.resource::<Entity>();

    // Find the down stairs and place the player
    let stair_type = if offset < 0 { TileType::DownStairs } else { TileType::UpStairs };
    {
        for (idx, _tile) in map.get_tile_type(stair_type).iter().enumerate() {
            let mut player_position = ecs.resource_mut::<Point>();
            *player_position = map.index_to_point2d(idx);

            if let Some(mut player_pos_comp) = ecs.get_mut::<Point>(player) {
                *player_pos_comp = map.index_to_point2d(idx);
                if new_depth == 1 {
                    player_pos_comp.x -= 1;
                }
            }
        }
    }

    let mut worldmap_resource = ecs.resource_mut::<Map>();
    *worldmap_resource = map;

    if let Some(mut fov) = ecs.get_mut::<FieldOfView>(player) {
        fov.is_dirty = true;
    }
}

pub struct MapGenPlugin;
impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::NewGame, setup_new_game.exclusive_system());

        app.add_enter_system(AppState::NextLevel, (goto_level).exclusive_system());
        app.add_enter_system(AppState::PreviousLevel, (goto_level).exclusive_system());
    }
}
