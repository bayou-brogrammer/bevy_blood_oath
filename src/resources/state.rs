use bevy::ecs::schedule::StateData;

use super::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    MainMenu,
    SetupDungeon,
    ShowInventory,
    GameOver,

    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    // Player Stages
    PlayerActions,
    PlayerCleanup,
    // AI Stages
    GenerateAIMoves,
    AIActions,
    AICleanup,
    // Render Is the last stage
    Render,
}

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
