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

pub struct DungeonMode {
    render_schedule: Schedule,
}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { f.debug_struct("DungeonMode").finish() }
}

////////////////////////////////////////////////////////////////////////////////

impl DungeonMode {
    pub fn new(app: &mut App) -> Self {
        raws::load_raws();

        // Create a render schedule and a stage
        let mut render_schedule = Schedule::default();
        let mut update = SystemStage::parallel();

        update.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<GameCamera>()
                .with_system(render::map_renderer::map_render)
                .with_system(render::entity_renderer::entity_render)
                .with_system(render::tooltips::render_tooltips)
                .into(),
        );

        render_schedule.add_stage(CoreStage::Update, update);

        // Setup State
        app.insert_resource(NextState(GameCondition::Playing));
        app.insert_resource(TurnState::AwaitingInput);

        // Setup Plugins
        app.add_plugin(SystemsPlugin);
        app.add_plugin(spawner::SpawnerPlugin);

        Self { render_schedule }
    }

    pub fn tick(
        &mut self,
        _ctx: &mut BTerm,
        app: &mut App,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        // Update Systems
        app.update();

        if let Some(result) = pop_result {
            match result {
                // App Quit
                ModeResult::AppQuitDialogModeResult(result) => match result {
                    AppQuitDialogModeResult::Cancelled => {}
                    AppQuitDialogModeResult::Confirmed => {
                        // if let Err(e) = saveload::save_game(world) {
                        //     eprintln!("Warning: bo_saveload::save_game: {}", e);
                        // }
                        return (ModeControl::Pop(DungeonModeResult::Done.into()), ModeUpdate::Immediate);
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
                            InventoryModeResult::EquipItem(item) => self.equip_item(&mut app.world, item),
                            InventoryModeResult::DropItem(item) => self.drop_item(&mut app.world, item),
                            InventoryModeResult::DropEquipment(item) => self.drop_item(&mut app.world, item),
                            InventoryModeResult::UseItem(item, target) => {
                                self.use_item(&mut app.world, item, *target)
                            }
                            InventoryModeResult::RemoveEquipment(equipment) => {
                                self.remove_equipment(&mut app.world, equipment)
                            }
                            _ => {}
                        }

                        self.end_turn(&mut app.world);
                    }
                },
                _ => unreachable!("Unknown popped dungeon result: [{:?}]", result),
            };
        }

        let turn_state = *app.world.resource::<TurnState>();
        match turn_state {
            TurnState::AwaitingInput => {
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
                        _ => {}
                    }
                }
            }
            TurnState::MagicMapReveal(row) => self.reveal_map(&mut app.world, row),
            _ => {}
        }

        // let turn_state = *app.world.resource::<TurnState>();
        // match turn_state {
        //     TurnState::MagicMapReveal(row) => self.reveal_map(&mut app.world, row),
        //     TurnState::AwaitingInput => match player_input2(ctx, &mut app.world) {
        //         PlayerInputResult::NoResult => {}
        //         PlayerInputResult::AppQuit => return self.app_quit_dialog(),
        //         PlayerInputResult::TurnDone => self.end_turn(&mut app.world),
        //         PlayerInputResult::ShowInventory => {
        //             return (
        //                 ModeControl::Push(InventoryMode::new(&mut app.world).into()),
        //                 ModeUpdate::Update,
        //             )
        //         }
        //         _ => {
        //             println!("DungeonMode::tick: Unknown player input result");
        //         }
        //     },
        //     _ => {}
        // }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, _ctx: &mut BTerm, world: &mut World, _active: bool) {
        self.render_schedule.run(world);
        gui::render_ui(world);
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
        println!("DungeonMode::use_item: {:?}", item);
        let p = *world.resource::<Entity>();
        world.send_event(WantsToUseItem(p, *item, pt));
    }

    fn drop_item(&self, world: &mut World, item: &Entity) {
        let p = *world.resource::<Entity>();
        world.send_event(WantsToDropItem(p, *item));
    }

    fn equip_item(&self, world: &mut World, equipment: &Entity) {
        let p = *world.resource::<Entity>();
        world.send_event(WantsToEquipItem(p, *equipment));
    }

    fn remove_equipment(&self, world: &mut World, equipment: &Entity) {
        let p = *world.resource::<Entity>();
        world.send_event(WantsToRemoveItem(p, *equipment));
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
            self.end_turn(world);
        } else {
            *runwriter = TurnState::MagicMapReveal(row + 1);
        }
    }
}
