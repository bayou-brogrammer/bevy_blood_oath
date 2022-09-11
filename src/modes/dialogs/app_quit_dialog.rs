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
}

impl State for AppQuitDialogMode {
    type State = GameWorld;
    type StateResult = ModeResult;

    fn update(
        &mut self,
        term: &mut BTerm,
        state: &mut Self::State,
        pop_result: &Option<Self::StateResult>,
    ) -> StateReturn<Self::State, Self::StateResult> {
        match self.dialog.update(term, state, pop_result) {
            (Transition::Pop(ModeResult::YesNoDialogModeResult(result)), mode_update) => match result {
                YesNoDialogModeResult::Yes => {
                    (Transition::Pop(AppQuitDialogModeResult::Confirmed.into()), mode_update)
                }
                YesNoDialogModeResult::No => {
                    (Transition::Pop(AppQuitDialogModeResult::Cancelled.into()), mode_update)
                }
            },
            result => result,
        }
    }

    fn render(&mut self, term: &mut BTerm, state: &mut Self::State, active: bool) {
        self.dialog.render(term, state, active);
    }
}
