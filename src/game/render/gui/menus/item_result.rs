use super::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    UpSelection,
    DownSelection,
    Selected(Entity),
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

    draw_batch.set(
        Point::new(x, y),
        ColorPair::new(WHITE, BLACK),
        to_cp437('('),
    );
    draw_batch.set(Point::new(x + 1, y), ColorPair::new(color, BLACK), hotkey);
    draw_batch.set(
        Point::new(x + 2, y),
        ColorPair::new(WHITE, BLACK),
        to_cp437(')'),
    );
    draw_batch.print_color(
        Point::new(x + 5, y),
        &text.to_string(),
        ColorPair::new(color, BLACK),
    );
}

pub fn item_result_menu<S: ToString>(
    draw_batch: &mut DrawBatch,
    title: S,
    count: usize,
    items: &[(Entity, String)],
    key: Option<VirtualKeyCode>,
    selection: usize,
) -> ItemMenuResult {
    let max_width = if count > 0 {
        items.iter().map(|s| s.1.len()).max().unwrap() + 8
    } else {
        20 // Base width for empty menu
    };

    let max_height = if count > 0 { count + 3 } else { 2 };

    let box_rect = center_box_with_title(
        draw_batch,
        (*MAP_PANEL_WIDTH, *MAP_PANEL_HEIGHT),
        BoxConfigWithTitle {
            box_config: BoxConfig::new(
                (max_width, max_height),
                ColorPair::new(WHITE, BLACK),
                true,
                false,
            ),
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

    if items.len() <= 0 {
        draw_batch.print_color_centered_at(
            Point::new(x + box_rect.width() / 2, y + box_rect.height() / 2),
            "<Empty>",
            ColorPair::new(WHITE, BLACK),
        );
    }

    for (j, item) in items.iter().enumerate() {
        menu_option(
            draw_batch,
            x + 1,
            y + 2,
            97 + j as FontCharType,
            &item.1,
            selection == j,
        );
        y += 1;
    }

    if let Some(key) = key {
        match key {
            VirtualKeyCode::Escape => ItemMenuResult::Cancel,
            VirtualKeyCode::Up => ItemMenuResult::UpSelection,
            VirtualKeyCode::Down => ItemMenuResult::DownSelection,
            key => {
                if key == VirtualKeyCode::Return {}

                let selection = letter_to_option(key);
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
