use crate::prelude::*;

use bevy_ecs::{
    schedule::{ShouldRun, StateData},
    system::Res,
};

////////////////////////////////////////////////////////////////////////////////
/// Run Criteria
////////////////////////////////////////////////////////////////////////////////
pub fn run_in_game_state(current: Res<TurnState>) -> bool {
    if current.stack.is_empty() {
        return false;
    }

    !current.stack.iter().any(|s| *s == TurnState::GameOver)
}

pub fn run_in_stack(state: TurnState) -> impl Fn(Res<StateStack<TurnState>>) -> bool + Clone + 'static {
    move |current: Res<StateStack<TurnState>>| -> bool {
        if current.stack.is_empty() {
            return false;
        }

        current.stack.iter().any(|s| *s == state)
    }
}

pub fn run_not_in_stack<T: StateData>(state: T) -> impl Fn(Res<StateStack<T>>) -> bool + Clone + 'static {
    move |current: Res<StateStack<T>>| -> bool {
        if current.stack.is_empty() {
            return false;
        }

        !current.stack.iter().any(|s| *s == state)
    }
}

pub fn run_in_stack_bevy<T: StateData>(
    state: T,
) -> impl Fn(Res<StateStack<T>>) -> ShouldRun + Clone + 'static {
    move |current: Res<StateStack<T>>| -> ShouldRun {
        if current.stack.is_empty() {
            return ShouldRun::No;
        }

        match current.stack.iter().any(|s| *s == state) {
            true => ShouldRun::Yes,
            false => ShouldRun::No,
        }
    }
}

pub fn run_not_in_stack_bevy<T: StateData>(
    state: T,
) -> impl Fn(Res<StateStack<T>>) -> ShouldRun + Clone + 'static {
    move |current: Res<StateStack<T>>| -> ShouldRun {
        if current.stack.is_empty() {
            return ShouldRun::No;
        }

        match current.stack.iter().any(|s| *s == state) {
            true => ShouldRun::No,
            false => ShouldRun::Yes,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
