use super::*;

use super::{ModeControl, ModeResult, ModeUpdate};

const YES_STR: &str = "[ Yes ]";
const NO_STR: &str = "[ No ]";

pub enum YesNoDialogModeResult {
    AppQuit,
    Yes,
    No,
}

pub struct YesNoDialogMode {
    prompt: String,
    yes_selected: bool,
}

impl From<bool> for YesNoDialogModeResult {
    fn from(yes: bool) -> Self {
        if yes {
            Self::Yes
        } else {
            Self::No
        }
    }
}

/// A yes-or-no dialog box with a prompt that shows up in the center of the screen.
impl YesNoDialogMode {
    pub fn new(prompt: String, yes_default: bool) -> Self {
        Self {
            prompt,
            yes_selected: yes_default,
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut BTerm,
        _world: &World,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        (ModeControl::Stay, ModeUpdate::WaitForEvent)
    }

    pub fn draw(&self, ctx: &mut BTerm, _world: &World, active: bool) {}
}
