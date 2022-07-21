use super::*;

pub fn map_render(camera: Res<GameCamera>, map: Res<Map>) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_MAP);

    camera.viewport.for_each(|pt| {
        if map.in_bounds(pt) && map.revealed.get_bit(pt) {
            let idx = map.point2d_to_index(pt);
            let t = &map.tiles[idx];

            let tint = if map.visible.get_bit(pt) {
                GREEN
            } else {
                DARK_GRAY
            };

            let color = ColorPair::new(tint, t.color.bg);
            batch.set(camera.world_to_screen(pt), color, t.glyph);
        }
    });

    batch.submit(0).expect("Error batching map");
}
