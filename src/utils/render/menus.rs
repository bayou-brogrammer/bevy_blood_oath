use super::*;

pub trait ActionMenu<T> {
    fn actions() -> Vec<T>;
    fn label(&self) -> &'static str;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ItemMenuResult<T> {
    Cancel,
    NoResponse,
    UpSelection,
    DownSelection,
    Selected(T),
}

pub fn menu_option<T: ToString>(
    draw_batch: &mut DrawBatch,
    x: i32,
    y: i32,
    hotkey: FontCharType,
    text: T,
    selected: bool,
) {
    let color = if selected { LIGHTBLUE } else { WHITE };

    draw_batch.set(Point::new(x, y), ColorPair::new(WHITE, BLACK), to_cp437('('));
    draw_batch.set(Point::new(x + 1, y), ColorPair::new(color, BLACK), hotkey);
    draw_batch.set(Point::new(x + 2, y), ColorPair::new(WHITE, BLACK), to_cp437(')'));
    draw_batch.print_color(Point::new(x + 5, y), &text.to_string(), ColorPair::new(color, BLACK));
}

pub fn item_result_menu<S: ToString>(
    draw_batch: &mut DrawBatch,
    dimensions: (i32, i32),
    title: S,
    count: i32,
    items: &[(Entity, String)],
    key: Option<&VirtualKeyCode>,
    selection: usize,
) -> ItemMenuResult<Entity> {
    let title_len = title.to_string().len() as i32 + 3;
    let max_width = if count > 0 {
        i32::max(title_len, (items.iter().map(|s| s.1.len()).max().unwrap() + 8) as i32)
    } else {
        i32::max(title_len, 20) // Base width for empty menu
    };

    let max_height = if count > 0 { count + 3 } else { 2 };
    let (screen_w, screen_h) = dimensions;

    let box_rect = center_box_with_title(
        draw_batch,
        (screen_w, screen_h),
        BoxConfigWithTitle {
            box_config: BoxConfig::new((max_width, max_height), ColorPair::new(WHITE, BLACK), true, false),
            text_config: TextConfig::with_footer(
                title,
                "[Esc] to cancel",
                ColorPair::new(CYAN, BLACK),
                ColorPair::new(YELLOW, BLACK),
                Alignment::Left,
            ),
        },
    );

    let x = box_rect.x1;
    let mut y = box_rect.y1;

    if items.is_empty() {
        draw_batch.print_color_centered_at(
            Point::new(x + box_rect.width() / 2, y + box_rect.height() / 2),
            "<Empty>",
            ColorPair::new(WHITE, BLACK),
        );
    }

    for (j, item) in items.iter().enumerate() {
        menu_option(draw_batch, x + 1, y + 2, 97 + j as FontCharType, &item.1, selection == j);
        y += 1;
    }

    if let Some(control) = key.get_key() {
        match control {
            GameKey::Escape => ItemMenuResult::Cancel,
            GameKey::Up => ItemMenuResult::UpSelection,
            GameKey::Down => ItemMenuResult::DownSelection,
            control => {
                let selection = if control == GameKey::Select {
                    selection as i32
                } else {
                    let key = key.unwrap();
                    letter_to_option(*key)
                };

                if selection > -1 && selection < count as i32 {
                    return ItemMenuResult::Selected(items[selection as usize].0);
                }

                ItemMenuResult::NoResponse
            }
        }
    } else {
        ItemMenuResult::NoResponse
    }
}
