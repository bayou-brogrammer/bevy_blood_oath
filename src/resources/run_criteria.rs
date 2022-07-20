use super::*;
use bevy::ecs::schedule::StateData;

pub fn run_in_state<T: StateData>(
    state: T,
) -> impl Fn(Res<StateStack<T>>) -> bool + Clone + 'static {
    move |current: Res<StateStack<T>>| -> bool {
        if current.stack.is_empty() {
            return false;
        }

        current.stack.iter().any(|s| *s == state)
    }
}
