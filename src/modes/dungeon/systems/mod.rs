use super::*;

pub mod end_turn;
pub mod fov;
pub mod inventory;
pub mod map_indexing;
pub mod melee_combat;
pub mod monster_ai;
pub mod movement;
pub mod particle_system;
pub mod player;
pub mod render;

pub struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .label(StateLabel::Fov)
                .run_unless_resource_equals(TurnState::GameOver)
                .with_system(fov::fov_system)
                .into(),
        );

        app.add_system(particle_system::particle_spawn_system);
        app.add_system(particle_system::update_particles);

        app.add_system_set_to_stage(
            GameStage::Cleanup,
            ConditionSet::new().with_system(map_indexing::map_indexing).into(),
        );

        // Inventory Events
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_on_event::<WantsToPickupItem>()
                .with_system(inventory::item_collection)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new().run_on_event::<WantsToUseItem>().with_system(inventory::item_use).into(),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new().run_on_event::<WantsToDropItem>().with_system(inventory::item_drop).into(),
        );
    }
}

pub struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_if(run_in_state(TurnState::AwaitingInput))
                .with_system(player::player_input.chain(player::player_turn_done))
                .into(),
        );
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::GeneratePlayerActions,
            ConditionSet::new()
                .run_if(run_in_state(TurnState::PlayerTurn))
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .into(),
        );

        app.add_system_set_to_stage(
            GameStage::HandlePlayerActions,
            ConditionSet::new()
                .run_if(run_in_state(TurnState::PlayerTurn))
                .with_system(map_indexing::map_indexing)
                .with_system(fov::fov_system)
                .with_system(end_turn::end_turn)
                .into(),
        );
    }
}

pub struct AIPlugin;
impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::GenerateAIActions,
            ConditionSet::new()
                .run_if(run_in_state(TurnState::AITurn))
                .with_system(monster_ai::monster_ai)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::HandleAIActions,
            ConditionSet::new()
                .run_if(run_in_state(TurnState::AITurn))
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::AICleanup,
            ConditionSet::new()
                .run_if(run_in_state(TurnState::AITurn))
                .with_system(map_indexing::map_indexing)
                .with_system(fov::fov_system)
                .with_system(end_turn::end_turn)
                .into(),
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TickingPlugin)
            .add_plugin(AwaitingInputPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AIPlugin);
    }
}
