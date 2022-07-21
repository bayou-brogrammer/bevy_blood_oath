use super::{dungeon::DungeonMode, ModeControl, ModeResult, *};

pub const MAIN_MENU_SCREEN_WIDTH: usize = 80;
pub const MAIN_MENU_SCREEN_HEIGHT: usize = 31;

#[derive(Debug)]
pub enum GameOverModeResult {
    AppQuit,
}

#[derive(Debug)]
pub enum MenuAction {
    NewGame,
    Quit,
}

impl MenuAction {
    fn label(&self) -> &'static str {
        match self {
            MenuAction::NewGame => "New Game",
            MenuAction::Quit => "Quit",
        }
    }
}

#[derive(Debug)]
pub struct GameOverMode {
    selection: usize,
    actions: Vec<MenuAction>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl GameOverMode {
    pub fn new() -> Self {
        BTerm::cls_all();

        let mut actions = vec![MenuAction::NewGame];

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(MenuAction::Quit);

        Self {
            actions,
            selection: 0,
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm, _pop_result: &Option<ModeResult>) -> ModeControl {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    return ModeControl::Pop(GameOverModeResult::AppQuit.into())
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
                        MenuAction::NewGame => {
                            return ModeControl::Switch(DungeonMode::new().into());
                        }
                        MenuAction::Quit => {
                            return ModeControl::Pop(GameOverModeResult::AppQuit.into())
                        }
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

        let box_rect = bo_utils::prelude::center_box_with_title(
            &mut batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfigWithTitle {
                box_config: BoxConfig::new((30, 20), ColorPair::new(WHITE, BLACK), true, false),
                text_config: TextConfig::new(
                    "GameOver",
                    ColorPair::new(RED, BLACK),
                    Alignment::Center,
                ),
            },
        );

        let mut y = MAIN_MENU_SCREEN_HEIGHT / 2 - 10;
        batch.print_color_centered(
            y + 1,
            "Use Up/Down Arrows and Enter",
            ColorPair::new(RGB::named(GRAY), RGB::named(BLACK)),
        );

        y = box_rect.center().y as usize - 2;
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
