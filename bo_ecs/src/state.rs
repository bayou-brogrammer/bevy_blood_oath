use bevy_ecs::{
    schedule::{ShouldRun, StateData},
    system::Res,
};

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

pub fn run_in_state_bevy<T: StateData>(
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

pub fn run_not_in_state_bevy<T: StateData>(
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
