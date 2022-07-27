use bevy_ecs::{
    schedule::{ShouldRun, StateData, StateError},
    system::Res,
};
use iyes_loopless::state::CurrentState;

////////////////////////////////////////////////////////////////////////////////
/// Run Criteria
////////////////////////////////////////////////////////////////////////////////

pub fn run_in_state_bevy<T: StateData>(
    state: T,
) -> impl Fn(Res<CurrentState<T>>) -> ShouldRun + Clone + 'static {
    move |current: Res<CurrentState<T>>| -> ShouldRun {
        if current.0 == state {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }
}

pub fn run_in_stack<T: StateData>(state: T) -> impl Fn(Res<StateStack<T>>) -> bool + Clone + 'static {
    move |stack: Res<StateStack<T>>| -> bool {
        if stack.stack.is_empty() {
            return false;
        }

        stack.stack.iter().all(|stack_state| *stack_state == state)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct StateStack<T: StateData> {
    stack: Vec<T>,
}

impl<T> StateStack<T>
where
    T: StateData,
{
    pub fn new(initial: T) -> Self {
        Self { stack: vec![initial] }
    }

    pub fn current(&self) -> &T {
        self.stack.last().unwrap()
    }
    pub fn set(&mut self, next_state: T) -> Result<(), StateError> {
        if self.stack.last().unwrap() == &next_state {
            return Err(StateError::AlreadyInState);
        }

        *self.stack.last_mut().unwrap() = next_state;
        Ok(())
    }

    pub fn push(&mut self, push_state: T) -> Result<(), StateError> {
        if self.stack.last().unwrap() == &push_state {
            return Err(StateError::AlreadyInState);
        }

        self.stack.push(push_state);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Option<T>, StateError> {
        if self.stack.len() == 1 {
            return Err(StateError::StackEmpty);
        }

        Ok(self.stack.pop())
    }
}
