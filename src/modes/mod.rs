use crate::prelude::*;

pub mod app_quit_dialog;
pub mod dungeon;
pub mod title;
pub mod yes_no_dialog;

use app_quit_dialog::{AppQuitDialogMode, AppQuitDialogModeResult};
use dungeon::{DungeonMode, DungeonModeResult};
use title::{TitleMode, TitleModeResult};
use yes_no_dialog::{YesNoDialogMode, YesNoDialogModeResult};

/// Return value for `update` callback sent into [run] that controls the main event loop.
pub enum RunControl {
    /// Wait for an event before calling `update` again.
    WaitForEvent,
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

pub enum Mode {
    AppQuitDialogMode(AppQuitDialogMode),
    DungeonMode(DungeonMode),
    TitleMode(TitleMode),
    YesNoDialogMode(YesNoDialogMode),
}

impl_from!(Mode, AppQuitDialogMode);
impl_from!(Mode, DungeonMode);
impl_from!(Mode, TitleMode);
impl_from!(Mode, YesNoDialogMode);

///////////////////////////////////////////////////////////////////////////////

pub enum ModeResult {
    AppQuitDialogModeResult(AppQuitDialogModeResult),
    DungeonModeResult(DungeonModeResult),
    TitleModeResult(TitleModeResult),
    YesNoDialogModeResult(YesNoDialogModeResult),
}

impl_from!(ModeResult, AppQuitDialogModeResult);
impl_from!(ModeResult, DungeonModeResult);
impl_from!(ModeResult, TitleModeResult);
impl_from!(ModeResult, YesNoDialogModeResult);

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

pub enum ModeUpdate {
    /// Run the next update immediately, without waiting for the next frame.
    Immediate,
    /// Wait a frame before the next update; this will likely draw the mode for a frame.
    Update,
    /// Wait for an input event before the next update; this will likely draw the mode before
    /// waiting.
    WaitForEvent,
}

impl Mode {
    fn update(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        match self {
            Mode::AppQuitDialogMode(x) => x.update(ctx, world, pop_result),
            Mode::DungeonMode(x) => x.update(ctx, world, pop_result),
            Mode::TitleMode(x) => x.update(ctx, world, pop_result),
            Mode::YesNoDialogMode(x) => x.update(ctx, world, pop_result),
        }
    }

    fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        match self {
            Mode::AppQuitDialogMode(x) => x.draw(ctx, world, active),
            Mode::DungeonMode(x) => x.draw(ctx, world, active),
            Mode::TitleMode(x) => x.draw(ctx, world, active),
            Mode::YesNoDialogMode(x) => x.draw(ctx, world, active),
        }
    }

    /// Should the current mode draw modes behind it in the stack?
    fn draw_behind(&self) -> bool {
        match self {
            Mode::AppQuitDialogMode(_) => true,
            Mode::DungeonMode(_) => false,
            Mode::TitleMode(_) => false,
            Mode::YesNoDialogMode(_) => true,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// The mode stack proper.  Create one of these with an initial mode, then call [ModeStack::update]
/// and [ModeStack::draw] at the appropriate points in the surrounding code; the mode stack and the
/// modes it holds will handle everything else.
pub struct ModeStack {
    stack: Vec<Mode>,
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
    pub fn update(&mut self, ctx: &mut BTerm, world: &mut World) -> RunControl {
        while !self.stack.is_empty() {
            // Update the top mode.
            let (mode_control, mode_update) = {
                let top_mode = self.stack.last_mut().unwrap();
                top_mode.update(ctx, world, &self.pop_result)
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
            if !self.stack.is_empty() && !matches!(mode_update, ModeUpdate::Immediate) {
                let draw_from = self
                    .stack
                    .iter()
                    .rposition(|mode| !mode.draw_behind())
                    .unwrap_or(0);
                let top = self.stack.len().saturating_sub(1);

                // Draw non-top modes with `active` set to `false`.
                for mode in self.stack.iter_mut().skip(draw_from) {
                    mode.draw(ctx, world, false);
                }

                // Draw top mode with `active` set to `true`.
                self.stack[top].draw(ctx, world, true);
            }

            match mode_update {
                ModeUpdate::Immediate => (),
                ModeUpdate::Update => return RunControl::Update,
                ModeUpdate::WaitForEvent => return RunControl::WaitForEvent,
            }
        }

        RunControl::Quit
    }
}
