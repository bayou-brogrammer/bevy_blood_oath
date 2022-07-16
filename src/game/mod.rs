use crate::prelude::*;

mod damage;
mod end_turn;
mod fov;
mod map_indexing;
mod melee_combat;
mod monster_ai;
mod movement;
mod player;

/**
 * We need multiple stages to handle the following:
 * 1. Update all events live in the world
 * 2. Handle input from player and generate actions
 * 3. Handle Player Turn
 * 4. Genrate Player Actions
 * 5. Handle AI Turn
 */
pub fn setup_scheduler(world: &mut World) -> Schedule {
    // Events
    world.insert_resource(Events::<WantsToMove>::default());
    world.insert_resource(Events::<WantsToAttack>::default());
    world.insert_resource(Events::<SufferDamage>::default());

    let mut schedule = Schedule::default();

    schedule
        .add_stage(GameStage::Update, SystemStage::parallel())
        .add_stage_after(
            GameStage::Update,
            GameStage::PlayerStage,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::PlayerStage,
            GameStage::GenerateAIMoves,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::GenerateAIMoves,
            GameStage::AIStage,
            SystemStage::parallel(),
        );

    // Updating Events System
    schedule.add_system_set_to_stage(
        GameStage::Update,
        ConditionSet::new()
            .with_system(Events::<WantsToMove>::update_system)
            .with_system(Events::<WantsToAttack>::update_system)
            .with_system(Events::<SufferDamage>::update_system)
            .with_system(map_indexing::map_indexing)
            .with_system(damage::damage_system)
            .into(),
    );

    // Awaiting Input System Set
    schedule.add_system_set_to_stage(
        GameStage::Update,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::AwaitingInput)
            .with_system(player::player_input)
            .with_system(fov::fov_system)
            .into(),
    );

    // Player System Set
    schedule.add_system_set_to_stage(
        GameStage::PlayerStage,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::PlayerTurn)
            .with_system(movement::movement)
            .with_system(fov::fov_system)
            .with_system(melee_combat::combat)
            .with_system(end_turn::end_turn)
            .into(),
    );

    schedule.add_system_set_to_stage(
        GameStage::GenerateAIMoves,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::AITurn)
            .with_system(monster_ai::monster_ai)
            .into(),
    );

    schedule.add_system_set_to_stage(
        GameStage::AIStage,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::AITurn)
            .with_system(movement::movement)
            .with_system(fov::fov_system)
            .with_system(melee_combat::combat)
            .with_system(end_turn::end_turn)
            .into(),
    );

    schedule
}
