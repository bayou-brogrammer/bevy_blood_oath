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
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    // Player Stages
    GeneratePlayerActions,
    HandlePlayerActions,
    // AI Stages
    GenerateAIActions,
    HandleAIActions,
    AICleanup,
    Cleanup,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum StateLabel {
    Fov,
}
