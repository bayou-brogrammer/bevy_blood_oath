use bevy_ecs::{
    prelude::Entity,
    schedule::{StageLabel, SystemLabel},
};
use bo_utils::impl_new;
use bracket_geometry::prelude::Point;

mod bundle;
pub use bundle::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    GameOver,

    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,

    Inventory,
    ShowDropMenu,
    Targeting,
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

pub struct Mouse {
    pub pt: Point,
    pub pos: (i32, i32),
    pub left_click: bool,
}

impl_new!(Mouse, pt: Point, pos: (i32, i32), left_click: bool);
