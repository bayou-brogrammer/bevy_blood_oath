use bevy_ecs::schedule::StateData;

#[derive(Debug, Clone)]
pub struct StateStack<T: StateData> {
    pub stack: Vec<T>,
}

// State
impl<T> StateStack<T>
where
    T: StateData + Copy,
{
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
