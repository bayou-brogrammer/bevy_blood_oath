use super::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
    PlayerCombat,
    PlayerCleanup,
    // AI Stages
    GenerateAIMoves,
    AICombat,
    AICleanup,
    // Render Is the last stage
    Render,
}
