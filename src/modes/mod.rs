use crate::{prelude::*, GameWorld};
use bracket_state_machine::{impl_from, prelude::*};

pub type ModeReturn = StateReturn<GameWorld, ModeResult>;

pub mod dialogs;
pub mod dungeon_mode;
pub mod game_over_mode;
pub mod inventory_mode;
pub mod main_menu_mode;
pub mod map_gen;
pub mod menu_memory;
pub mod targeting_mode;

use dungeon_mode::{DungeonMode, DungeonModeResult};
use game_over_mode::GameOverModeResult;
use inventory_mode::EquipmentActionModeResult;
use inventory_mode::InventoryActionModeResult;
use inventory_mode::InventoryModeResult;
use main_menu_mode::MainMenuModeResult;
pub use menu_memory::MenuMemory;
use targeting_mode::{TargetingMode, TargetingModeResult};

use dialogs::*;

#[derive(Debug)]
pub enum ModeResult {
    DungeonModeResult(DungeonModeResult),
    MainMenuModeResult(MainMenuModeResult),
    GameOverModeResult(GameOverModeResult),
    InventoryModeResult(InventoryModeResult),
    TargetingModeResult(TargetingModeResult),
    MessageBoxModeResult(MessageBoxModeResult),
    YesNoDialogModeResult(YesNoDialogModeResult),
    AppQuitDialogModeResult(AppQuitDialogModeResult),
    InventoryActionModeResult(InventoryActionModeResult),
    EquipmentActionModeResult(EquipmentActionModeResult),
}

impl_from!(ModeResult, DungeonModeResult);
impl_from!(ModeResult, MainMenuModeResult);
impl_from!(ModeResult, GameOverModeResult);
impl_from!(ModeResult, InventoryModeResult);
impl_from!(ModeResult, TargetingModeResult);
impl_from!(ModeResult, MessageBoxModeResult);
impl_from!(ModeResult, YesNoDialogModeResult);
impl_from!(ModeResult, AppQuitDialogModeResult);
impl_from!(ModeResult, InventoryActionModeResult);
impl_from!(ModeResult, EquipmentActionModeResult);

/// Helper macro to convert a type into an enum variant with the same name.
#[macro_export]
macro_rules! impl_state_boxed {
    ($mode:ident, $state:ident, $result:ident) => {
        impl From<$mode>
            for Box<dyn bracket_state_machine::prelude::State<State = $state, StateResult = $result>>
        {
            fn from(mode: $mode) -> Self {
                Box::new(mode)
            }
        }
    };
}

impl_state_boxed!(DungeonMode, GameWorld, ModeResult);
