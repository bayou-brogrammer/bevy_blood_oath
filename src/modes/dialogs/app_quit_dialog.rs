use super::*;

#[derive(Debug)]
pub enum AppQuitDialogModeResult {
    Cancelled,
    Confirmed,
}

#[derive(Debug, Default)]
pub struct AppQuitDialogMode {
    dialog: YesNoDialogMode,
}

/// A yes-or-no dialog box that appears when the use requests that the app be closed.
impl AppQuitDialogMode {
    pub fn new() -> Self {
        Self { dialog: YesNoDialogMode::new("Really quit Bload Oath?".to_string(), false) }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        app: &mut App,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        match self.dialog.tick(ctx, app, pop_result) {
            (ModeControl::Pop(ModeResult::YesNoDialogModeResult(result)), mode_update) => match result {
                YesNoDialogModeResult::Yes => {
                    (ModeControl::Pop(AppQuitDialogModeResult::Confirmed.into()), mode_update)
                }
                YesNoDialogModeResult::No => {
                    (ModeControl::Pop(AppQuitDialogModeResult::Cancelled.into()), mode_update)
                }
            },
            result => result,
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, app: &mut App, active: bool) {
        self.dialog.draw(ctx, app, active);
    }
}
