use super::*;

#[system]
#[read_component(Position)]
#[read_component(Description)]
#[read_component(Name)]
pub fn render_tooltips(ecs: &SubWorld, #[resource] map: &Map, #[resource] mouse_pos: &Point) {
    let Point { x: mx, y: my } = mouse_pos;
    let map_x = mx - 1;
    let map_y = my - 1;

    if map_x >= 0 && map_x < WIDTH as i32 && map_y >= 0 && map_y < HEIGHT as i32 {
        let mut lines = Vec::new();
        let mut query = <(&Position, &Description)>::query();
        query.for_each(ecs, |(pos, desc)| {
            if pos.layer == map.current_layer && pos.pt.x == map_x && pos.pt.y == map_y {
                let idx = map.get_current().point2d_to_index(pos.pt);
                if map.get_current().visible[idx] {
                    lines.push(desc.0.clone());
                }
            }
        });

        if !lines.is_empty() {
            let mut batch = DrawBatch::new();
            let height = lines.len() + 2;
            let width = lines.iter().map(|s| s.len()).max().unwrap() + 2;

            let tip_x = if map_x < WIDTH as i32 / 2 {
                mx + 1
            } else {
                mx - (width as i32 + 1)
            };
            let tip_y = if map_y > HEIGHT as i32 / 2 {
                my - height as i32
            } else {
                *my
            };

            batch.draw_box(
                Rect::with_size(tip_x, tip_y, width as i32, height as i32),
                ColorPair::new(WHITE, BLACK),
            );

            let mut y = tip_y + 1;
            lines.iter().for_each(|s| {
                gui::safe_print_color(
                    &mut batch,
                    Point::new(tip_x + 1, y),
                    s,
                    ColorPair::new(WHITE, BLACK),
                );
                y += 1;
            });

            batch.submit(100_000).expect("Error batching tooltips");
        }
    }
}
