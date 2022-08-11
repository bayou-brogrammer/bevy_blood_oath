use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum GameCondition {
    MainMenu,
    Setup,
    Playing,
    GameOver,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,
    MagicMapReveal(i32),
    Dead,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum PlayerStage {
    GenerateActions,
    HandleActions,
    Cleanup,
    Effects,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum AIStage {
    HandleAI,
    GenerateActions,
    HandleActions,
    Cleanup,
    Effects,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum StateLabel {
    Fov,
    Indexing,
    AIActions,
}
