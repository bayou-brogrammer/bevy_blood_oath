use bevy_ecs::{
    prelude::Entity,
    schedule::{StageLabel, SystemLabel},
};

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

    Inventory,
    ShowDropMenu,
    Targeting { range: i32, item: Entity },
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
