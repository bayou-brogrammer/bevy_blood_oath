use super::*;

const YES_STR: &str = "[ Yes ]";
const NO_STR: &str = "[ No ]";

pub fn confirm_input(
    mut commands: Commands,
    mut yes_selected: Local<bool>,
    key: Option<Res<VirtualKeyCode>>,
    stack: Res<StateStack<TurnState>>,
) -> (bool, String) {
    if let Some(key) = key.as_deref() {
        match key {
            VirtualKeyCode::Left => *yes_selected = true,
            VirtualKeyCode::Right => *yes_selected = false,
            VirtualKeyCode::Return => {
                commands.remove_resource::<VirtualKeyCode>();
                commands.insert_resource(YesNoDialog(*yes_selected));
                commands.insert_resource(PopState);
            }
            _ => {}
        }
    }

    let prompt = match stack.current() {
        TurnState::Confirm(prompt) => prompt.clone(),
        _ => "".to_string(),
    };

    (*yes_selected, prompt.clone())
}

pub fn confirm(In((yes_selected, prompt)): In<(bool, String)>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_MAP);

    let box_rect = center_box(
        &mut draw_batch,
        (MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT),
        BoxConfig::new((prompt.len() as i32 + 3, 10), ColorPair::new(WHITE, BLACK), true, false),
    );

    let (x, y) = (box_rect.x1, box_rect.y1);

    // Prompt
    draw_batch.print_color_centered_at(
        Point::new(x + box_rect.width() / 2 + 1, y + 1),
        prompt.clone(),
        ColorPair::new(WHITE, BLACK),
    );

    // Yes/No
    let yes_x = box_rect.width() - (YES_STR.len() + NO_STR.len() + 4) as i32;
    let no_x = box_rect.width() - NO_STR.len() as i32 - 2;

    draw_batch.print_color_centered_at(
        Point::new(x + yes_x, y + 3),
        YES_STR,
        ColorPair::new(WHITE, if yes_selected { bo_utils::SELECTED_BG } else { BLACK }),
    );
    draw_batch.print_color_centered_at(
        Point::new(x + no_x, y + 3),
        NO_STR,
        ColorPair::new(WHITE, if !yes_selected { bo_utils::SELECTED_BG } else { BLACK }),
    );

    draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
}
