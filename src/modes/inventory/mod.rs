use super::{dungeon::DungeonMode, ModeControl, ModeResult, *};

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum InventoryModeResult {
    AppQuit,
    DoNothing,
    DropItem(Entity),
    EquipItem(Entity),
    DropEquipment(Entity),
    RemoveEquipment(Entity),
    UseItem(Entity, Option<(i32, i32)>),
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum SubSection {
    EquipWeapon,
    EquipArmor,
    SortAll,
    Inventory,
}

#[derive(Debug)]
pub struct InventoryMode {
    inv_selection: i32,
    subsection: SubSection,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl InventoryMode {
    pub fn new() -> Self {
        BTerm::clear_all_internal_consoles();

        Self { inv_selection: 0, subsection: SubSection::Inventory }
    }

    pub fn tick(&mut self, ctx: &mut BTerm, _pop_result: &Option<ModeResult>) -> ModeControl {
        ModeControl::Stay
    }

    pub fn draw(&self, _ctx: &mut BTerm, _active: bool) {}
}
