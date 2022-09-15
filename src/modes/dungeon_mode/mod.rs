use super::*;
use crate::inventory_mode::InventoryMode;

mod player;
pub use player::*;

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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DungeonMode").finish()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl DungeonMode {
    pub fn new(app: &mut App) -> Self {
        // Create a render schedule and a stage
        let mut render_schedule = Schedule::default();
        let mut update = SystemStage::parallel();

        update.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<CameraView>()
                .with_system(render::map_renderer::map_render)
                .with_system(render::entity_renderer::entity_render)
                .with_system(render::entity_renderer::particle_render)
                // .with_system(render::tooltips::render_tooltips)
                .into(),
        );

        render_schedule.add_stage(CoreStage::Update, update);

        // Setup State
        app.insert_resource(TurnState::AwaitingInput);
        app.insert_resource(NextState(GameCondition::Playing));

        // Setup Plugins
        app.add_plugin(SystemsPlugin);
        app.add_plugin(spawner::SpawnerPlugin);

        Self { render_schedule }
    }
}

impl State for DungeonMode {
    type State = GameWorld;
    type StateResult = ModeResult;

    fn update(
        &mut self,
        term: &mut BTerm,
        state: &mut Self::State,
        pop_result: &Option<Self::StateResult>,
    ) -> ModeReturn {
        // Update Systems
        state.app.update();

        if let Some(result) = pop_result {
            let world = &mut state.app.world;

            match result {
                // App Quit
                ModeResult::AppQuitDialogModeResult(result) => match result {
                    AppQuitDialogModeResult::Cancelled => {}
                    AppQuitDialogModeResult::Confirmed => {
                        // if let Err(e) = saveload::save_game(world) {
                        //     eprintln!("Warning: bo_saveload::save_game: {}", e);
                        // }
                        return (
                            Transition::Pop(DungeonModeResult::Done.into()),
                            TransitionControl::Immediate,
                        );
                    }
                },

                // Yes / No Dialog
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::No => {}
                    YesNoDialogModeResult::Yes => {
                        return (
                            Transition::Switch(MapGenMode::next_level(world).boxed()),
                            TransitionControl::Immediate,
                        );
                    }
                },

                // Inventory
                ModeResult::InventoryModeResult(result) => match result {
                    InventoryModeResult::DoNothing => {}
                    _ => {
                        match result {
                            InventoryModeResult::EquipItem(item) => self.equip_item(world, item),
                            InventoryModeResult::DropItem(item) => self.drop_item(world, item),
                            InventoryModeResult::DropEquipment(item) => self.drop_item(world, item),
                            InventoryModeResult::UseItem(item, target) => self.use_item(world, item, *target),
                            InventoryModeResult::RemoveEquipment(equipment) => {
                                self.remove_equipment(world, equipment)
                            }
                            _ => {}
                        }

                        self.end_turn(world);
                    }
                },
                _ => unreachable!("Unknown popped dungeon result: [{:?}]", result),
            };
        }

        let turn_state = *state.app.world.resource::<TurnState>();
        match turn_state {
            TurnState::MagicMapReveal(row) => self.reveal_map(&mut state.app.world, row),
            TurnState::AwaitingInput => {
                match player_input(term, &mut state.app.world) {
                    PlayerInputResult::NoResult => {}
                    PlayerInputResult::AppQuit => return self.app_quit_dialog(),
                    PlayerInputResult::TurnDone => self.end_turn(&mut state.app.world),
                    PlayerInputResult::ShowInventory => {
                        return (
                            Transition::Push(InventoryMode::new(&mut state.app.world).boxed()),
                            TransitionControl::Update,
                        )
                    }
                    player::PlayerInputResult::Descend => {
                        return (
                            Transition::Push(
                                YesNoDialogMode::new("Descend to the next level?".to_string(), false).boxed(),
                            ),
                            TransitionControl::Update,
                        );
                    }
                    _ => {}
                }
                // if let Some(result) = state.app.world.remove_resource::<PlayerInputResult>() {
                //     match result {
                //         PlayerInputResult::NoResult => {}
                //         PlayerInputResult::AppQuit => return self.app_quit_dialog(),
                //         PlayerInputResult::TurnDone => self.end_turn(&mut state.app.world),
                //         PlayerInputResult::ShowInventory => {
                //             return (
                //                 Transition::Push(InventoryMode::new(&mut state.app.world).boxed()),
                //                 TransitionControl::Update,
                //             )
                //         }
                //         _ => {}
                //     }
                // }
            }
            _ => {}
        }

        (Transition::Stay, TransitionControl::Update)
    }

    fn render(&mut self, _term: &mut BTerm, state: &mut Self::State, _active: bool) {
        self.render_schedule.run(&mut state.app.world);
        gui::render_ui(&mut state.app.world);
    }
}

impl DungeonMode {
    fn app_quit_dialog(&self) -> ModeReturn {
        #[cfg(not(target_arch = "wasm32"))]
        return (Transition::Push(AppQuitDialogMode::new().boxed()), TransitionControl::Update);

        #[cfg(target_arch = "wasm32")]
        return (Transition::Stay, TransitionControl::Update);
    }

    fn end_turn(&self, world: &mut World) {
        bo_logging::record_event(TURN_DONE_EVENT, 1);
        let mut runwriter = world.resource_mut::<TurnState>();
        *runwriter = TurnState::PlayerTurn
    }

    fn use_item(&self, world: &mut World, item: &Entity, pt: Option<Point>) {
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
