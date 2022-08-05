use self::player::PlayerInputResult;
use crate::prelude::*;

pub mod end_turn;
pub mod fov;
pub mod hunger;
pub mod inventory;
pub mod map_indexing;
pub mod melee_combat;
pub mod monster_ai;
pub mod movement;
pub mod particles;
pub mod player;

pub struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(particles::ParticlePlugin);
        app.add_plugin(inventory::InventoryPlugin);

        app.add_system_set(
            ConditionSet::new()
                .label(StateLabel::Fov)
                .run_in_state(GameCondition::Playing)
                .with_system(fov::fov_system)
                .with_system(map_indexing::map_indexing)
                .into(),
        );
    }
}

pub struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AwaitingInput)
                .with_system(player::player_input.chain(
                    |In(result): In<PlayerInputResult>, mut commands: Commands| {
                        commands.insert_resource(result)
                    },
                ))
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
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .with_system(inventory::item_use)
                .with_system(hunger::hunger_clock)
                .into(),
        );

        app.add_system_set_to_stage(
            GameStage::HandlePlayerActions,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(map_indexing::map_indexing)
                .with_system(fov::fov_system)
                .with_system(end_turn::end_turn)
                .into(),
        );

        app.add_system_set_to_stage(
            GameStage::HandlePlayerActions,
            SystemSet::new()
                .with_run_criteria(run_in_state_bevy(GameCondition::Playing))
                .with_system(run_effects_queue.exclusive_system()),
        );
    }
}

pub struct AIPlugin;
impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::GenerateAIActions,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(monster_ai::monster_ai)
                .with_system(hunger::hunger_clock)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::HandleAIActions,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::AICleanup,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(map_indexing::map_indexing)
                .with_system(fov::fov_system)
                .with_system(end_turn::end_turn)
                .into(),
        );

        app.add_system_set_to_stage(
            GameStage::AICleanup,
            SystemSet::new()
                .with_run_criteria(run_in_state_bevy(GameCondition::Playing))
                .with_system(run_effects_queue.exclusive_system()),
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
