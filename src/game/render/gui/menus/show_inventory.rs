use super::*;

pub fn show_inventory(
    items_q: Query<(Entity, &Naming, &InBackpack), With<Item>>,
    player: Query<Entity, With<Player>>,
    key: Res<Option<VirtualKeyCode>>,
    mut state: ResMut<TurnState>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_TEXT);

    let player = player.single();

    let mut items: Vec<(Entity, String)> = Vec::new();
    items_q
        .iter()
        .filter(|(_, _, backpack)| backpack.0 == player)
        .for_each(|(item, item_name, _)| items.push((item, item_name.0.clone())));

    match item_result_menu(&mut draw_batch, "Inventory", items.len(), &items, *key) {
        (ItemMenuResult::Cancel, _) => *state = TurnState::AwaitingInput,
        (ItemMenuResult::Selected, item) => {
            let item_entity = item;
        }
        _ => {}
    }

    draw_batch.submit(50_000).expect("Batch error"); // On top of everything
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn menu_option<T: ToString>(
    draw_batch: &mut DrawBatch,
    x: i32,
    y: i32,
    hotkey: FontCharType,
    text: T,
) {
    draw_batch.set(
        Point::new(x, y),
        ColorPair::new(WHITE, BLACK),
        to_cp437('('),
    );
    draw_batch.set(Point::new(x + 1, y), ColorPair::new(YELLOW, BLACK), hotkey);
    draw_batch.set(
        Point::new(x + 2, y),
        ColorPair::new(WHITE, BLACK),
        to_cp437(')'),
    );
    draw_batch.print_color(
        Point::new(x + 5, y),
        &text.to_string(),
        ColorPair::new(YELLOW, BLACK),
    );
}

pub fn item_result_menu<S: ToString>(
    draw_batch: &mut DrawBatch,
    title: S,
    count: usize,
    items: &[(Entity, String)],
    key: Option<VirtualKeyCode>,
) -> (ItemMenuResult, Option<Entity>) {
    let mut y = (25 - (count / 2)) as i32;

    draw_batch.draw_box(
        Rect::with_size(15, y - 2, 31, (count + 3) as i32),
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color(
        Point::new(18, y - 2),
        &title.to_string(),
        ColorPair::new(YELLOW, BLACK),
    );
    draw_batch.print_color(
        Point::new(18, y + count as i32 + 1),
        "ESCAPE to cancel",
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_list: Vec<Entity> = Vec::new();
    let mut j = 0;
    for item in items {
        menu_option(draw_batch, 17, y, 97 + j as FontCharType, &item.1);
        item_list.push(item.0);
        y += 1;
        j += 1;
    }

    match key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(item_list[selection as usize]),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}
