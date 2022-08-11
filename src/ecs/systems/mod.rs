use bevy::ecs::schedule::ShouldRun;

use self::player::PlayerInputResult;
use crate::prelude::*;

pub mod ai;
pub mod end_turn;
pub mod fov;
pub mod hunger;
pub mod inventory;
pub mod map_indexing;
pub mod melee_combat;
pub mod movement;
pub mod particles;
pub mod player;
pub mod render;
pub mod trigger;

pub struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(particles::ParticlePlugin);
        app.add_plugin(inventory::InventoryPlugin);

        // This is our pre-run systems
        app.add_enter_system_set(
            GameCondition::Playing,
            SystemSet::new()
                .with_system(map_indexing::map_indexing.label(StateLabel::Indexing))
                .with_system(fov::fov_system.after(StateLabel::Indexing)),
        );

        app.add_system_set(
            ConditionSet::new()
                .label(StateLabel::Fov)
                .run_in_state(GameCondition::Playing)
                .with_system(fov::fov_system)
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
        // Generate Actions
        app.add_system_set_to_stage(
            PlayerStage::GenerateActions,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .with_system(inventory::item_use)
                .with_system(hunger::hunger_clock)
                .into(),
        )
        // Handle Actions
        .add_system_set_to_stage(
            PlayerStage::HandleActions,
            ConditionSet::new()
                .label(StateLabel::Indexing)
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(map_indexing::map_indexing)
                .with_system(trigger::triggers)
                .into(),
        )
        .add_system_set_to_stage(
            PlayerStage::Effects,
            SystemSet::new()
                .with_run_criteria(run_in_state_bevy(GameCondition::Playing))
                .with_run_criteria(run_if_queue)
                .with_system(run_effects_queue.exclusive_system()),
        )
        // CLeanup
        .add_system_set_to_stage(
            PlayerStage::Cleanup,
            ConditionSet::new()
                .label(StateLabel::Fov)
                .after(StateLabel::Indexing)
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(fov::fov_system)
                .with_system(end_turn::end_turn)
                .into(),
        );
    }
}

pub struct AIPlugin;
impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        // Generate Actions
        app.add_system_set_to_stage(
            AIStage::HandleAI,
            ConditionSet::new()
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(ai::monster_ai)
                .with_system(ai::bystander_ai)
                .into(),
        )
        // Handle Actions
        .add_system_set_to_stage(
            AIStage::GenerateActions,
            ConditionSet::new()
                .label(StateLabel::AIActions)
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .with_system(hunger::hunger_clock)
                .into(),
        )
        // Cleanup
        .add_system_set_to_stage(
            AIStage::HandleActions,
            ConditionSet::new()
                .label(StateLabel::Indexing)
                .after(StateLabel::AIActions)
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(map_indexing::map_indexing)
                .with_system(trigger::triggers)
                .into(),
        )
        .add_system_set_to_stage(
            AIStage::Effects,
            SystemSet::new()
                .with_run_criteria(run_in_state_bevy(GameCondition::Playing))
                .with_run_criteria(run_if_queue)
                .with_system(run_effects_queue.exclusive_system()),
        )
        .add_system_set_to_stage(
            AIStage::Cleanup,
            ConditionSet::new()
                .label(StateLabel::Fov)
                .after(StateLabel::Indexing)
                .run_in_state(GameCondition::Playing)
                .run_if_resource_equals(TurnState::AITurn)
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

fn run_if_queue() -> ShouldRun {
    if EFFECT_QUEUE.lock().is_empty() {
        ShouldRun::No
    } else {
        ShouldRun::YesAndCheckAgain
    }
}
