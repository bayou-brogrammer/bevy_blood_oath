use super::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    MainMenu,
    SetupDungeon,
    ShowInventory,

    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    PlayerStage,
    GenerateAIMoves,
    AIStage,
    Render,
    RenderBatch,
}
