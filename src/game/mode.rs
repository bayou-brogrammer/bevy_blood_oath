use crate::prelude::*;

use dungeon::{DungeonMode, DungeonModeResult};
use game_over::{GameOverMode, GameOverModeResult};
use main_menu::{MainMenuMode, MainMenuModeResult};

/// Return value for `update` callback sent into [run] that controls the main event loop.
pub enum RunControl {
    /// Call `update` again next frame.
    Update,
    /// Quit the run loop.
    Quit,
}

/// Helper macro to convert a type into an enum variant with the same name.
macro_rules! impl_from {
    ($to:ty, $from:ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                Self::$from(f)
            }
        }
    };
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Mode {
    // AppQuitDialogMode(AppQuitDialogMode),
    DungeonMode(DungeonMode),
    MainMenuMode(MainMenuMode),
    GameOverMode(GameOverMode),
}

impl_from!(Mode, DungeonMode);
impl_from!(Mode, MainMenuMode);
impl_from!(Mode, GameOverMode);

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ModeResult {
    // AppQuitDialogModeResult(AppQuitDialogModeResult),
    DungeonModeResult(DungeonModeResult),
    MainMenuModeResult(MainMenuModeResult),
    GameOverModeResult(GameOverModeResult),
}

// impl_from!(ModeResult, AppQuitDialogModeResult);
impl_from!(ModeResult, DungeonModeResult);
impl_from!(ModeResult, MainMenuModeResult);
impl_from!(ModeResult, GameOverModeResult);

///////////////////////////////////////////////////////////////////////////////

pub enum ModeControl {
    /// Keep the stack as-is.
    Stay,
    /// Replace the current mode on the stack with a new mode.
    Switch(Mode),
    /// Push a new mode on top of the current mode on the stack.
    Push(Mode),
    /// Pop the current mode from the stack, with a corresponding result.
    Pop(ModeResult),
    /// Clear the whole stack, while returning a corresponding result.
    Terminate(ModeResult),
}

impl Mode {
    fn tick(&mut self, ctx: &mut BTerm, pop_result: &Option<ModeResult>) -> ModeControl {
        match self {
            Mode::DungeonMode(x) => x.tick(ctx, pop_result),
            Mode::MainMenuMode(x) => x.tick(ctx, pop_result),
            Mode::GameOverMode(x) => x.tick(ctx, pop_result),
        }
    }

    fn draw(&mut self, ctx: &mut BTerm, active: bool) {
        match self {
            Mode::MainMenuMode(x) => x.draw(ctx, active),
            Mode::DungeonMode(x) => x.draw(ctx, active),
            Mode::GameOverMode(x) => x.draw(ctx, active),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// The mode stack proper.  Create one of these with an initial mode, then call [ModeStack::update]
/// and [ModeStack::draw] at the appropriate points in the surrounding code; the mode stack and the
/// modes it holds will handle everything else.
#[derive(Debug)]
pub struct ModeStack {
    pub stack: Vec<Mode>,
    pop_result: Option<ModeResult>,
}

impl ModeStack {
    /// Create a new mode stack.
    pub fn new(stack: Vec<Mode>) -> Self {
        Self {
            stack,
            pop_result: None,
        }
    }

    /// Perform update logic for the top mode of the stack, and then drawing logic for all  modes.
    ///
    /// This also converts [ModeUpdate] values into [ruggrogue::RunControl] values to control the
    /// behavior of the next update.
    pub fn tick(&mut self, ctx: &mut BTerm) -> RunControl {
        while !self.stack.is_empty() {
            // Update the top mode.
            let mode_control = {
                let top_mode = self.stack.last_mut().unwrap();
                top_mode.tick(ctx, &self.pop_result)
            };

            self.pop_result = None;

            // Control the stack as requested by the top mode update logic.
            match mode_control {
                ModeControl::Stay => {}
                ModeControl::Switch(mode) => {
                    self.stack.pop();
                    self.stack.push(mode);
                }
                ModeControl::Push(mode) => {
                    self.stack.push(mode);
                }
                ModeControl::Pop(mode_result) => {
                    self.pop_result = Some(mode_result);
                    self.stack.pop();
                }
                ModeControl::Terminate(mode_result) => {
                    self.pop_result = Some(mode_result);
                    self.stack.clear();
                }
            }

            // Draw modes in the stack from the bottom-up.
            if !self.stack.is_empty() {
                let top = self.stack.len().saturating_sub(1);

                // Draw top mode with `active` set to `true`.
                self.stack[top].draw(ctx, true);
            }

            return RunControl::Update;
        }

        RunControl::Quit
    }
}
