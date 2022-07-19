use super::{
    dungeon::DungeonMode,
    mode::{ModeControl, ModeResult},
    *,
};

pub const MAIN_MENU_SCREEN_WIDTH: usize = 80;
pub const MAIN_MENU_SCREEN_HEIGHT: usize = 31;

#[derive(Debug)]
pub enum MainMenuModeResult {
    AppQuit,
}

#[derive(Debug)]
pub enum TitleAction {
    NewGame,
    LoadGame,
    Options,
    Quit,
}

impl TitleAction {
    fn label(&self) -> &'static str {
        match self {
            TitleAction::NewGame => "New Game",
            TitleAction::LoadGame => "Load Game",
            TitleAction::Options => "Options",
            TitleAction::Quit => "Quit",
        }
    }
}

#[derive(Debug)]
pub struct MainMenuMode {
    selection: usize,
    actions: Vec<TitleAction>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MainMenuMode {
    pub fn new() -> Self {
        let mut actions = vec![TitleAction::NewGame];

        // There's no obvious way to get Emscripten to load the IndexedDB filesystem in time to
        // realize that a save file exists, so always include the Load Game option for it and just
        // check if there really is a save file when the option is chosen instead.
        if cfg!(target_os = "emscripten") {
            actions.push(TitleAction::LoadGame);
        }

        actions.push(TitleAction::Options);

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(TitleAction::Quit);

        Self {
            actions,
            selection: 0,
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm, _pop_result: &Option<ModeResult>) -> ModeControl {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    return ModeControl::Pop(MainMenuModeResult::AppQuit.into())
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
                        TitleAction::NewGame => {
                            return ModeControl::Switch(DungeonMode::new().into());
                        }
                        TitleAction::Quit => {
                            return ModeControl::Pop(MainMenuModeResult::AppQuit.into())
                        }
                        _ => {} // Don't Handle loading or options yet.
                    }
                }
                _ => {}
            }
        }

        ModeControl::Stay
    }

    pub fn draw(&self, _ctx: &mut BTerm, _active: bool) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_TEXT);
        batch.cls();

        batch.draw_hollow_double_box(
            Rect::with_exact(
                MAIN_MENU_SCREEN_WIDTH / 2 - 20,
                MAIN_MENU_SCREEN_HEIGHT / 2 - 10,
                MAIN_MENU_SCREEN_WIDTH / 2 + 20,
                MAIN_MENU_SCREEN_HEIGHT / 2,
            ),
            ColorPair::new(DARK_GRAY, BLACK),
        );

        let mut y = MAIN_MENU_SCREEN_HEIGHT / 2 - 10;
        batch.print_color_centered(
            y,
            "BloodOath",
            ColorPair::new(RGB::named(RED), RGB::named(BLACK)),
        );
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

        y += 4;
        for (i, action) in self.actions.iter().enumerate() {
            let color = if i == self.selection {
                RGB::named(MAGENTA)
            } else {
                RGB::named(GRAY)
            };

            batch.print_color_centered(
                y + i,
                action.label(),
                ColorPair::new(color, RGB::named(BLACK)),
            );
        }

        batch.submit(0).expect("Error batching title");
    }
}
