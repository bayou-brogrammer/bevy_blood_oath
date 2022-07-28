use bevy_ecs::{
    prelude::Entity,
    schedule::{StageLabel, SystemLabel},
};
use bo_utils::impl_new;
use bracket_geometry::prelude::Point;

mod bundle;
mod run_criteria;

pub use bundle::*;
pub use run_criteria::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum GameCondition {
    MainMenu,
    GameOver,
    InGame,
    LoadGame,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum TurnState {
    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,

    Inventory,
    ShowDropMenu,
    Targeting,
    Confirm(String),
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

#[derive(Debug)]
pub struct Targeting {
    pub range: i32,
    pub item: Entity,
}

impl_new!(Targeting, item: Entity, range: i32);

pub struct MousePosition {
    pub pt: Point,
    pub pos: (i32, i32),
}

pub struct MouseLeftClick(pub bool);

impl_new!(MousePosition, pt: Point, pos: (i32, i32));

#[derive(Debug)]
pub struct YesNoDialog(pub bool);

///////////////////////////////////////////////////////////////////////////////////////////////
/// State Management
///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct PopState;
#[derive(Debug)]
pub struct PushState(pub TurnState);

#[derive(Debug)]
pub struct SetState(pub TurnState);
