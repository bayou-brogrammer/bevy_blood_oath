use super::{ModeControl, ModeResult, *};

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MainMenuModeResult {
    AppQuit,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MainMenuAction {
    NewGame,
    Quit,
}

impl MainMenuAction {
    fn label(&self) -> &'static str {
        match self {
            MainMenuAction::NewGame => "New Game",
            MainMenuAction::Quit => "Quit",
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct MainMenuMode {
    selection: usize,
    actions: Vec<MainMenuAction>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MainMenuMode {
    pub fn new() -> Self {
        let mut actions = vec![MainMenuAction::NewGame];

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(MainMenuAction::Quit);

        Self { actions, selection: 0 }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        app: &mut App,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        app.update();

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    return (ModeControl::Pop(MainMenuModeResult::AppQuit.into()), ModeUpdate::Immediate)
                }
                VirtualKeyCode::Down => {
                    if self.selection < self.actions.len().saturating_sub(1) {
                        self.selection += 1;
                    } else {
                        self.selection = 0;
                    }
                }
                VirtualKeyCode::Up => {
                    if self.selection > 0 {
                        self.selection -= 1;
                    } else {
                        self.selection = self.actions.len().saturating_sub(1);
                    }
                }
                VirtualKeyCode::Return => {
                    assert!(self.selection < self.actions.len());

                    match self.actions[self.selection] {
                        MainMenuAction::Quit => {
                            return (
                                ModeControl::Pop(MainMenuModeResult::AppQuit.into()),
                                ModeUpdate::Immediate,
                            )
                        }
                        MainMenuAction::NewGame => {
                            return (
                                ModeControl::Switch(MapGenMode::new_game(&mut app.world).into()),
                                ModeUpdate::Immediate,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, ctx: &mut BTerm, world: &mut World, _active: bool) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_ZERO);

        let assets = world.resource::<RexAssets>();
        ctx.render_xp_sprite(&assets.menu, 0, 0);

        let box_rect = center_box_with_title(
            &mut batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfigWithTitle {
                box_config: BoxConfig::new((30, 10), ColorPair::new(WHITE, BLACK), true, false),
                text_config: TextConfig::new("BloodOath", ColorPair::new(RED, BLACK), Alignment::Center),
            },
        );

        let mut y = box_rect.y1 + 1;
        batch.print_color_centered(
            y + 1,
            "by Jacob LeCoq",
            ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
        );
        batch.print_color_centered(
            y + 2,
            "Use Up/Down Arrows and Enter",
            ColorPair::new(RGB::named(GRAY), RGB::named(BLACK)),
        );

        y = box_rect.center().y;
        for (i, action) in self.actions.iter().enumerate() {
            let color = if i == self.selection { RGB::named(MAGENTA) } else { RGB::named(GRAY) };

            batch.print_color_centered(
                y + i as i32,
                action.label(),
                ColorPair::new(color, RGB::named(BLACK)),
            );
        }

        batch.submit(LAYER_ZERO).expect("Error batching title");
    }
}
