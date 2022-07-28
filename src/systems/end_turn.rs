use crate::prelude::*;

pub fn end_turn(
    state: Res<TurnState>,
    mut commands: Commands,
    player_stats_q: Query<&CombatStats, With<Player>>,
) {
    let stats = player_stats_q.single();

    let current_state = *state;
    let new_state = match current_state {
        TurnState::PlayerTurn => TurnState::AITurn,
        TurnState::AITurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    if stats.hp < 1 {
        commands.insert_resource(NextState(GameCondition::GameOver));
        return;
    }

    commands.insert_resource(new_state);
}
