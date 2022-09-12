use super::*;
use bracket_state_machine::prelude::TransitionControl;

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
}

impl State for MainMenuMode {
    type State = GameWorld;
    type StateResult = ModeResult;

    fn update(
        &mut self,
        term: &mut BTerm,
        state: &mut Self::State,
        _pop_result: &Option<Self::StateResult>,
    ) -> ModeReturn {
        state.app.update();

        if let Some(key) = term.key {
            match key {
                VirtualKeyCode::Escape => {
                    return (
                        Transition::Pop(MainMenuModeResult::AppQuit.into()),
                        TransitionControl::Immediate,
                    )
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
                                Transition::Pop(MainMenuModeResult::AppQuit.into()),
                                TransitionControl::Immediate,
                            )
                        }
                        MainMenuAction::NewGame => {
                            return (
                                Transition::Switch(MapGenMode::new_game(&mut state.app.world).boxed()),
                                TransitionControl::Immediate,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        (Transition::Stay, TransitionControl::Update)
    }

    fn render(&mut self, _term: &mut BTerm, _state: &mut Self::State, _active: bool) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_ZERO);

        let box_rect = center_box_with_title(
            &mut batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfigWithTitle {
                box_config: BoxConfig::new((30, 10), ColorPair::new(WHITE, BLACK), true, false),
                text_config: TextConfig::new(
                    "BloodOath",
                    ColorPair::new(RED, BLACK),
                    Alignment::Center,
                    true,
                ),
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
