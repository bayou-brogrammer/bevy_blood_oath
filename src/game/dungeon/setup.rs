use super::*;

use super::systems::{
    damage, end_turn, fov, inventory, map_indexing, melee_combat, monster_ai, movement, player,
};

/**
 * We need multiple stages to handle the following:
 * 1. Update all events live in the world
 * 2. Handle input from player and generate actions
 * 3. Handle Player Turn
 * 4. Genrate Player Actions
 * 5. Handle AI Turn
 */
pub fn setup_dungeon_scheduler(app: &mut App) {
    setup_events(app);
    setup_stages(app);
    setup_bevy_internals(app);

    setup_turn_state_systems(app);
    setup_every_tick_systems(app);

    app.add_plugin(render::RenderPlugin);

    app.init_resource::<Time>();
    app.add_system(move |mut time: ResMut<Time>| time.update());
}

fn setup_bevy_internals(app: &mut App) {
    app.init_resource::<Time>();
    app.add_system(|mut time: ResMut<Time>| time.update());
}

fn setup_events(app: &mut App) {
    app.add_event::<WantsToMove>()
        .add_event::<WantsToAttack>()
        .add_event::<SufferDamage>()
        .add_event::<WantsToPickupItem>();
}

fn setup_stages(app: &mut App) {
    app.add_stage_after(
        CoreStage::Update,
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
    )
    .add_stage_after(
        CoreStage::PostUpdate,
        GameStage::Render,
        SystemStage::single_threaded(),
    )
    .add_stage_after(
        GameStage::Render,
        GameStage::RenderBatch,
        SystemStage::parallel(),
    );
}

fn setup_every_tick_systems(app: &mut App) -> &mut App {
    // Updating Events System
    app.add_system_set_to_stage(
        CoreStage::Update,
        ConditionSet::new()
            .with_system(map_indexing::map_indexing)
            .with_system(damage::damage_system)
            .with_system(inventory::item_collection)
            .into(),
    )
}

fn setup_turn_state_systems(app: &mut App) -> &mut App {
    app
        // Input Stages
        .add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AwaitingInput)
                .with_system(player::player_input.chain(player::player_turn_done))
                .with_system(fov::fov_system)
                .into(),
        )
        // Player Stages
        .add_system_set_to_stage(
            GameStage::PlayerStage,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(movement::movement)
                .with_system(fov::fov_system)
                .with_system(melee_combat::combat)
                .with_system(end_turn::end_turn)
                .into(),
        )
        // AI Stages
        .add_system_set_to_stage(
            GameStage::GenerateAIMoves,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(monster_ai::monster_ai)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::AIStage,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(movement::movement)
                .with_system(fov::fov_system)
                .with_system(melee_combat::combat)
                .with_system(end_turn::end_turn)
                .into(),
        )
}

// fn setup_inventory_menu_system_sets(schedule: &mut Schedule) {
//     // Generic State Systems
//     schedule.add_system_set_to_stage(
//         GameStage::Update,
//         ConditionSet::new()
//             .run_if_resource_equals(InternalGameState::ShowInventory)
//             .with_system(gui::show_inventory)
//             .into(),
//     );
// }
