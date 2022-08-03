use super::*;

pub fn map_render(camera: Res<GameCamera>, map: Res<Map>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_ZERO);

    let (min_x, max_x, min_y, max_y) = camera.get_screen_bounds();
    let map_width = map.width - 1;
    let map_height = map.height - 1;

    for (y, ty) in (min_y..max_y).enumerate() {
        for (x, tx) in (min_x..max_x).enumerate() {
            if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                let pt = Point::new(tx, ty);
                let idx = map.point2d_to_index(pt);

                if map.revealed.get_bit(pt) {
                    let (glyph, color) = tile_glyph(idx, &*map);
                    draw_batch.set(Point::new(x + 1, y + 1), color, glyph);
                }
            } else if SHOW_BOUNDARIES {
                draw_batch.set(Point::new(x + 1, y + 1), ColorPair::new(GRAY, BLACK), to_cp437('·'));
            }
        }
    }

    draw_batch.submit(BATCH_ZERO).expect("Error batching map");
}

pub fn map_render_debug(map_gen: Res<MapGenResource>, ctx: Res<BracketContext>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_ZERO);

    if let Some(map) = map_gen.mapgen_history.get(map_gen.mapgen_index) {
        let player_pos = Point::new(map.width / 2, map.height / 2);
        let (x_chars, y_chars) = ctx.char_size;

        let center_x = (x_chars / 2) as i32;
        let center_y = (y_chars / 2) as i32;

        let min_x = player_pos.x - center_x;
        let max_x = min_x + x_chars as i32;
        let min_y = player_pos.y - center_y;
        let max_y = min_y + y_chars as i32;

        let map_width = map.width - 1;
        let map_height = map.height - 1;

        // Render Map
        for (y, ty) in (min_y..max_y).enumerate() {
            for (x, tx) in (min_x..max_x).enumerate() {
                let pt = Point::new(tx, ty);
                if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                    let idx = map.point2d_to_index(pt);

                    if map.revealed.get_bit(pt) {
                        let (glyph, color) = tile_glyph(idx, &*map);
                        draw_batch.set(Point::new(x, y), color, glyph);
                    }
                } else if SHOW_BOUNDARIES {
                    draw_batch.set(Point::new(x, y), ColorPair::new(GRAY, BLACK), to_cp437('·'));
                }
            }
        }
    }

    draw_batch.submit(BATCH_ZERO).expect("Error batching map");
}
