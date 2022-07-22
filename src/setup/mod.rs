use crate::prelude::*;

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // Setup Scheduler
        setup_events(app);
        setup_stages(app);
        setup_debug_systems(app);

        app.add_plugin(SystemsPlugin);
        app.add_plugin(EffectsPlugin);

        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(run_in_stack_bevy(TurnState::SetupDungeon))
                .with_system(setup.exclusive_system()),
        );
    }
}

pub fn setup(world: &mut World) {
    let map = Map::new(0, MAPWIDTH as i32, MAPHEIGHT as i32, "Dungeon");
    let start_pos = map.starting_point;

    // Spawn Player
    spawner::spawn_player(world, start_pos);

    // Spawn Enemies
    map.rooms.iter().skip(1).for_each(|room| {
        spawner::spawn_room(world, room);
    });

    spawner::magic_missile_scroll(world, start_pos);
    spawner::fireball_scroll(world, start_pos);

    // Resource
    world.insert_resource(map);
    world.insert_resource(start_pos);
    world.insert_resource(ParticleBuilder::new());
    world.insert_resource(camera::GameCamera::new(start_pos));
    world.insert_resource(StateStack::new(TurnState::AwaitingInput));

    bo_logging::Logger::new().append("Welcome to").append_with_color("Rusty Roguelike", CYAN).log();
}

pub fn setup_events(app: &mut App) {
    // Intent Events
    app.add_event::<WantsToMove>();
    app.add_event::<WantsToAttack>();

    // Item Events
    app.add_event::<WantsToPickupItem>();
    app.add_event::<WantsToUseItem>();
    app.add_event::<WantsToDropItem>();

    // Effects
    app.add_event::<AffectEntity>();
    app.add_event::<AffectTile>();

    // Effect Events
    app.add_event::<DamageEvent>()
        .add_event::<DeathEvent>()
        .add_event::<HealEvent>()
        .add_event::<ParticleEvent>();

    // Trigger Events
    app.add_event::<ItemTrigger>();
}

/**
 * We need multiple stages to handle the following:
 * 1. Handle input from player and generate actions
 * 2. Generate Player Actions
 * 3. Handle Player Actions
 * 4. Generate AI Actions
 * 5. Handle AI Actions
 * 6. Effects
 */

pub fn setup_stages(app: &mut App) {
    // Player Stages
    app.add_stage_after(
        CoreStage::Update,
        GameStage::GeneratePlayerActions,
        SystemStage::parallel(),
    )
    .add_stage_after(
        GameStage::GeneratePlayerActions,
        GameStage::HandlePlayerActions,
        SystemStage::parallel(),
    )
    .add_stage_after(
        GameStage::HandlePlayerActions,
        GameStage::GenerateAIActions,
        SystemStage::parallel(),
    )
    // AI Stages
    .add_stage_after(
        GameStage::GenerateAIActions,
        GameStage::HandleAIActions,
        SystemStage::parallel(),
    )
    .add_stage_after(GameStage::HandleAIActions, GameStage::AICleanup, SystemStage::parallel())
    .add_stage_after(GameStage::HandleAIActions, GameStage::Effects, SystemStage::parallel())
    .add_stage_after(GameStage::Effects, GameStage::Cleanup, SystemStage::parallel());
}

pub fn setup_debug_systems(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::Update,
        ConditionSet::new()
            .with_system(
                |m_q: Query<&Position, Added<Monster>>, i_q: Query<&Position, Added<Item>>| {
                    for pos in m_q.iter() {
                        eprintln!("Monster Spawned at {:?}", pos)
                    }
                    for pos in i_q.iter() {
                        eprintln!("Item Spawned at {:?}", pos)
                    }
                },
            )
            .into(),
    );
}
