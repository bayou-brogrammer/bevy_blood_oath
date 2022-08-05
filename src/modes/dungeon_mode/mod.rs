use super::*;
use crate::player::PlayerInputResult;

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum DungeonModeResult {
    Done,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////
pub struct DungeonMode {}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DungeonMode").finish()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
///

impl DungeonMode {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        app: &mut App,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            match result {
                // App Quit
                ModeResult::AppQuitDialogModeResult(result) => match result {
                    AppQuitDialogModeResult::Cancelled => {}
                    AppQuitDialogModeResult::Confirmed => {
                        // if let Err(e) = saveload::save_game(world) {
                        //     eprintln!("Warning: bo_saveload::save_game: {}", e);
                        // }
                        return (
                            ModeControl::Pop(DungeonModeResult::Done.into()),
                            ModeUpdate::Immediate,
                        );
                    }
                },

                // Yes / No Dialog
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::No => {}
                    YesNoDialogModeResult::Yes => {
                        return (
                            ModeControl::Switch(MapGenMode::next_level(&mut app.world).into()),
                            ModeUpdate::Immediate,
                        );
                    }
                },

                // Inventory
                ModeResult::InventoryModeResult(result) => match result {
                    InventoryModeResult::DoNothing => {}
                    _ => {
                        match result {
                            InventoryModeResult::EquipItem(item) => {
                                self.use_item(&mut app.world, item, None)
                            }
                            InventoryModeResult::DropItem(item) => self.drop_item(&mut app.world, item),
                            InventoryModeResult::DropEquipment(item) => {
                                self.drop_item(&mut app.world, item)
                            }
                            InventoryModeResult::UseItem(item, target) => {
                                self.use_item(&mut app.world, item, *target)
                            }
                            InventoryModeResult::RemoveEquipment(equipment) => {
                                self.remove_equipment(&mut app.world, equipment)
                            }
                            _ => {}
                        }

                        let mut runwriter = app.world.resource_mut::<TurnState>();
                        *runwriter = TurnState::PlayerTurn;
                    }
                },
                _ => unreachable!("Unknown popped dungeon result: [{:?}]", result),
            };
        }

        if let Some(result) = app.world.remove_resource::<PlayerInputResult>() {
            match result {
                PlayerInputResult::NoResult => {}
                PlayerInputResult::AppQuit => return self.app_quit_dialog(),
                PlayerInputResult::TurnDone => self.end_turn(&mut app.world),
                PlayerInputResult::ShowInventory => {
                    return (
                        ModeControl::Push(InventoryMode::new(&mut app.world).into()),
                        ModeUpdate::Update,
                    )
                }
                _ => {
                    println!("DungeonMode::tick: Unknown player input result: {:?}", result);
                }
            }
        }

        // app.update();

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, _ctx: &mut BTerm, app: &mut App, _active: bool) {
        app.update();
    }
}

impl DungeonMode {
    fn app_quit_dialog(&self) -> (ModeControl, ModeUpdate) {
        #[cfg(not(target_arch = "wasm32"))]
        return (ModeControl::Push(AppQuitDialogMode::new().into()), ModeUpdate::Update);

        #[cfg(target_arch = "wasm32")]
        return (ModeControl::Stay, ModeUpdate::Update);
    }

    fn end_turn(&self, world: &mut World) {
        bo_logging::record_event(TURN_DONE_EVENT, 1);
        let mut runwriter = world.resource_mut::<TurnState>();
        *runwriter = TurnState::PlayerTurn
    }

    fn use_item(&self, world: &mut World, item: &Entity, pt: Option<Point>) {
        let p = *world.resource::<Entity>();
        let mut evw = world.resource_mut::<EventWriter<WantsToUseItem>>();
        evw.send(WantsToUseItem(p, *item, pt));
    }

    fn drop_item(&self, world: &mut World, item: &Entity) {
        let p = *world.resource::<Entity>();
        let mut evw = world.resource_mut::<EventWriter<WantsToDropItem>>();
        evw.send(WantsToDropItem(p, *item));
    }

    fn remove_equipment(&self, world: &mut World, equipment: &Entity) {
        let p = *world.resource::<Entity>();
        let mut evw = world.resource_mut::<EventWriter<WantsToRemoveItem>>();
        evw.send(WantsToRemoveItem(p, *equipment));
    }

    fn reveal_map(&self, world: &mut World, row: i32) {
        let height: i32;
        {
            let mut map = world.resource_mut::<Map>();
            height = map.height;

            for x in 0..map.width {
                let pt = Point::new(x as i32, row);
                map.revealed.set_bit(pt, true);
            }
        }

        let mut runwriter = world.resource_mut::<TurnState>();
        if row == height - 1 {
            *runwriter = TurnState::PlayerTurn
        } else {
            *runwriter = TurnState::MagicMapReveal(row + 1);
        }
    }
}
