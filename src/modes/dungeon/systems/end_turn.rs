use crate::prelude::*;

pub fn end_turn(
    player_stats_q: Query<&CombatStats, With<Player>>,
    mut stack: ResMut<StateStack<TurnState>>,
) {
    let stats = player_stats_q.single();

    let current_state = *stack.current();
    let mut new_state = match current_state {
        TurnState::PlayerTurn => TurnState::AITurn,
        TurnState::AITurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    if stats.hp < 1 {
        new_state = TurnState::GameOver;
    }

    // commands.insert_resource(new_state);
    stack.set(new_state);
}
