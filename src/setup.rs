use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;

use crate::{prelude::*, spawner::SpawnerPlugin};

pub fn setup_events(app: &mut App) {
    // Intent Events
    app.add_event::<WantsToMove>();
    app.add_event::<WantsToAttack>();
    app.add_event::<WantsToUseItem>();

    // Item Events
    app.add_event::<WantsToDropItem>();
    app.add_event::<WantsToEquipItem>();
    app.add_event::<WantsToPickupItem>();
    app.add_event::<WantsToRemoveItem>();
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
    app.add_stage_after(CoreStage::Update, GameStage::GeneratePlayerActions, SystemStage::parallel())
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
        .add_stage_after(GameStage::HandleAIActions, GameStage::Cleanup, SystemStage::parallel());
}

pub fn setup_debug_systems(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::Update,
        ConditionSet::new()
            .with_system(|m_q: Query<&Position, Added<Monster>>, i_q: Query<&Position, Added<Item>>| {
                for pos in m_q.iter() {
                    eprintln!("Monster Spawned at {:?}", pos)
                }
                for pos in i_q.iter() {
                    eprintln!("Item Spawned at {:?}", pos)
                }
            })
            .into(),
    );
}

fn setup_game(mut commands: Commands) {
    commands.insert_resource(ParticleBuilder::new());
    commands.insert_resource(TurnState::AwaitingInput);
    commands.insert_resource(ReportExecutionOrderAmbiguities);
    commands.insert_resource(Map::new(0, SCREEN_WIDTH, SCREEN_HEIGHT, "Dungeon"));

    bo_logging::Logger::new().append("Welcome to").append_with_color("Rusty Roguelike", CYAN).log();
}

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpawnerPlugin);

        app.add_startup_system(setup_game);

        // app.add_exit_system(GameCondition::MainMenu, setup_game);
        // app.add_exit_system(GameCondition::GameOver, setup_game);
    }
}
