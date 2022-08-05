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
    MapGen(MapGenState),
    Playing,
    GameOver,
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

#[derive(Debug)]
pub struct Targeting {
    pub range: i32,
    pub item: Entity,
}

pub struct BracketContext {
    pub mouse_pt: Point,
    pub frame_time_ms: f32,
    pub char_size: (u32, u32),
    pub mouse_pos: (i32, i32),
    pub mouse_left_click: bool,
}

impl_new!(Targeting, item: Entity, range: i32);
impl_new!(
    BracketContext,
    frame_time_ms: f32,
    char_size: (u32, u32),
    mouse_pos: (i32, i32),
    mouse_pt: Point,
    mouse_left_click: bool
);
