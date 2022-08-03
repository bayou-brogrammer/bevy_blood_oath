use crate::prelude::*;

mod bundle;
mod run_criteria;

pub use bundle::*;
pub use run_criteria::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum MapGenState {
    NewGame,
    NextLevel(usize),
    Generate,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum GameCondition {
    MainMenu,
    GameOver,
    InGame,
    MapGen(MapGenState),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    // Turn States
    AwaitingInput,
    PlayerTurn,
    AITurn,

    Targeting,
    Inventory,
    ShowDropMenu,
    ShowRemoveMenu,
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

#[derive(Debug)]
pub struct Targeting {
    pub range: i32,
    pub item: Entity,
}

pub struct MousePosition {
    pub pt: Point,
    pub pos: (i32, i32),
}

pub struct MouseLeftClick(pub bool);

pub struct BracketContext {
    pub frame_time_ms: f32,
    pub char_size: (u32, u32),
}

impl_new!(Targeting, item: Entity, range: i32);
impl_new!(MousePosition, pt: Point, pos: (i32, i32));
impl_new!(BracketContext, frame_time_ms: f32, char_size: (u32, u32));
