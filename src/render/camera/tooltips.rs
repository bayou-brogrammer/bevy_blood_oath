use super::*;

pub fn render_tooltips(
    map: Res<Map>,
    mouse: Res<Mouse>,
    camera: Res<GameCamera>,
    tooltip_q: Query<(&Position, &Naming, Option<&Description>, Option<&CombatStats>)>,
) {
    let (min_x, _max_x, min_y, _max_y) = camera.get_screen_bounds();

    let (mouse_x, mouse_y) = mouse.pos;
    let mut mouse_map_pos = Point::from(mouse.pos);
    mouse_map_pos.x += min_x - 1;
    mouse_map_pos.y += min_y - 1;

    if !map.in_bounds(mouse_map_pos) || !map.visible.get_bit(Point::from(mouse_map_pos)) {
        return;
    }

    let mut lines = Vec::new();
    tooltip_q
        .iter()
        .filter(|(pos, _, _, _)| pos.0 == mouse_map_pos)
        .for_each(|(pos, name, desc, stats)| {
            if map.visible.get_bit(pos.0) {
                lines.push((CYAN, name.0.clone()));

                if let Some(desc) = desc {
                    lines.push((GRAY, desc.0.clone()));
                }

                if let Some(stats) = stats {
                    lines.push((GRAY, format!("{}/{} hp", stats.hp, stats.max_hp)));
                }
            }
        });

    let mut batch = DrawBatch::new();

    if !lines.is_empty() {
        let height = lines.len() + 1;
        let width = lines.iter().map(|s| s.1.len()).max().unwrap() + 2;

        let tip_x = if mouse_map_pos.x < map.width as i32 / 2 {
            i32::min((mouse_x * 2) + 1, 111)
        } else {
            i32::max(0, (mouse_x * 2) - (width as i32 + 1))
        };
        let tip_y = if mouse_map_pos.y > map.height as i32 / 2 { mouse_y - height as i32 } else { mouse_y };

        batch.draw_box(
            Rect::with_size(tip_x, tip_y - (lines.len() / 2) as i32, width as i32, height as i32),
            ColorPair::new(WHITE, BLACK),
        );

        let mut y = tip_y + 1 - (lines.len() / 2) as i32;
        lines.iter().for_each(|s| {
            safe_print_color(&mut batch, Point::new(tip_x + 1, y), &s.1, ColorPair::new(s.0, BLACK));
            y += 1;
        });
    }

    batch.submit(BATCH_TOOLTIPS).expect("Error batching tooltips");
}
