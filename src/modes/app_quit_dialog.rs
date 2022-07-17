use super::*;

pub enum AppQuitDialogModeResult {
    Cancelled,
    Confirmed,
}

pub struct AppQuitDialogMode(YesNoDialogMode);

/// A yes-or-no dialog box that appears when the use requests that the app be closed.
impl AppQuitDialogMode {
    pub fn new() -> Self {
        Self(YesNoDialogMode::new(
            "Really quit RuggRogue?".to_string(),
            false,
        ))
    }

    pub fn update(
        &mut self,
        ctx: &mut BTerm,
        world: &World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        match self.0.update(ctx, world, pop_result) {
            (ModeControl::Pop(ModeResult::YesNoDialogModeResult(result)), mode_update) => {
                match result {
                    YesNoDialogModeResult::AppQuit => (ModeControl::Stay, ModeUpdate::WaitForEvent),
                    YesNoDialogModeResult::Yes => (
                        ModeControl::Pop(AppQuitDialogModeResult::Confirmed.into()),
                        mode_update,
                    ),
                    YesNoDialogModeResult::No => (
                        ModeControl::Pop(AppQuitDialogModeResult::Cancelled.into()),
                        mode_update,
                    ),
                }
            }
            result => result,
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, world: &World, active: bool) {
        self.0.draw(ctx, world, active);
    }
}
