use bevy_ecs::schedule::{StageLabel, SystemLabel};

mod criteria;
mod state;

pub use criteria::*;
pub use state::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    MainMenu,
    GameOver,
    SetupDungeon,

    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,

    ShowInventory,
    ShowDropMenu,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum StateLabel {
    Fov,
}
