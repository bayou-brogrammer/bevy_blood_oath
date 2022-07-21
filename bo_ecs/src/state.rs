#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct StateStack<T> {
    pub stack: Vec<T>,
}

// State
impl<T: StateData> StateStack<T> {
    pub fn new(state: T) -> Self {
        Self { stack: vec![state] }
    }

    pub fn push(&mut self, state: T) {
        self.stack.push(state);
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.stack.is_empty() {
            self.stack.pop()
        } else {
            None
        }
    }

    pub fn set(&mut self, state: T) {
        self.stack.clear();
        self.stack.push(state);
    }

    pub fn replace(&mut self, state: T) {
        self.stack.clear();
        self.stack.push(state);
    }

    pub fn current(&self) -> &T {
        self.stack.last().unwrap()
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Run Criteria
////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////

use bevy::{ecs::schedule::StateData, prelude::Res};
