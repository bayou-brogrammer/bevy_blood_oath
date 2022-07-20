use super::*;

pub mod damage;
pub mod end_turn;
pub mod fov;
pub mod inventory;
pub mod map_indexing;
pub mod melee_combat;
pub mod monster_ai;
pub mod movement;
pub mod player;

pub fn exit_on_esc_system(
    key: Res<Option<VirtualKeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(key) = key.as_ref() {
        if *key == VirtualKeyCode::Escape {
            app_exit_events.send(AppExit);
        }
    }
}

pub struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new().with_system(exit_on_esc_system).into(),
        );
    }
}

pub struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AwaitingInput)
                .with_system(player::player_input.chain(player::player_turn_done))
                .with_system(fov::fov_system)
                .into(),
        );
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::PlayerCombat,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .with_system(inventory::item_collection)
                .with_system(inventory::item_use)
                .into(),
        );

        app.add_system_set_to_stage(
            GameStage::PlayerCleanup,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::PlayerTurn)
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
            GameStage::GenerateAIMoves,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(monster_ai::monster_ai)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::AICombat,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(movement::movement)
                .with_system(melee_combat::combat)
                .into(),
        )
        .add_system_set_to_stage(
            GameStage::AICleanup,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::AITurn)
                .with_system(map_indexing::map_indexing)
                .with_system(fov::fov_system)
                .with_system(damage::damage_system)
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
