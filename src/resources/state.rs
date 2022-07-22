use super::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    MainMenu,
    SetupDungeon,
    ShowInventory,
    ShowDropMenu,
    GameOver,

    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,
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
    Effects,
    Cleanup,
    // Render Is the last stage
    Render,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum StateLabel {
    Fov,
}
