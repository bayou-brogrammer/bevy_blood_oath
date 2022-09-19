use super::*;
use crate::GameWorld;

const YES_STR: &str = "[ Yes ]";
const NO_STR: &str = "[ No ]";

#[derive(Debug)]
pub enum YesNoDialogModeResult {
    Yes,
    No,
}

#[derive(Debug, Default)]
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
        Self { prompt, yes_selected: yes_default }
    }
}

impl State for YesNoDialogMode {
    type State = GameWorld;
    type StateResult = ModeResult;

    fn update(
        &mut self,
        term: &mut BTerm,
        _state: &mut Self::State,
        _pop_result: &Option<Self::StateResult>,
    ) -> StateReturn<Self::State, Self::StateResult> {
        if let Some(key) = term.key {
            match key {
                VirtualKeyCode::Escape => {
                    return (Transition::Pop(YesNoDialogModeResult::No.into()), TransitionControl::Update);
                }
                VirtualKeyCode::Left => self.yes_selected = true,
                VirtualKeyCode::Right => self.yes_selected = false,
                VirtualKeyCode::Return => {
                    return (
                        Transition::Pop(YesNoDialogModeResult::from(self.yes_selected).into()),
                        TransitionControl::Update,
                    );
                }
                _ => {}
            }
        }

        (Transition::Stay, TransitionControl::Update)
    }

    fn render(&mut self, term: &mut BTerm, _state: &mut Self::State, _active: bool) {
        term.print(0, MAP_PANEL_HEIGHT / 2, "*");
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_TEXT);

        let box_rect = center_box(
            &mut draw_batch,
            (MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT),
            BoxConfig::new((self.prompt.len() as i32 + 3, 5), ColorPair::new(WHITE, BLACK), true, false),
        );

        let (x, y) = (box_rect.x1, box_rect.y1);

        // Prompt
        draw_batch.print_color_centered_at(
            Point::new(x + box_rect.width() / 2 + 1, y + 1),
            self.prompt.clone(),
            ColorPair::new(WHITE, BLACK),
        );

        // Yes/No
        let yes_x = box_rect.width() - (YES_STR.len() + NO_STR.len() + 4) as i32;
        let no_x = box_rect.width() - NO_STR.len() as i32 - 2;

        draw_batch.print_color_centered_at(
            Point::new(x + yes_x, y + 3),
            YES_STR,
            ColorPair::new(WHITE, if self.yes_selected { crate::utils::SELECTED_BG } else { BLACK }),
        );
        draw_batch.print_color_centered_at(
            Point::new(x + no_x, y + 3),
            NO_STR,
            ColorPair::new(WHITE, if !self.yes_selected { crate::utils::SELECTED_BG } else { BLACK }),
        );

        draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
    }
}
