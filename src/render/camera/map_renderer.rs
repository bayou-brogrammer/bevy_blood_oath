use super::*;

pub fn map_render(camera: Res<GameCamera>, map: Res<Map>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_MAP);

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
            }
        }
    }

    draw_batch.submit(BATCH_ZERO).expect("Error batching map");
}
