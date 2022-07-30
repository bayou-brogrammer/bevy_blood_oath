use super::*;

pub fn entity_render(map: Res<Map>, camera: Res<GameCamera>, glyph_q: Query<(&Position, &Glyph)>) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_ENTITY);

    let mut entities = glyph_q.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

    for (pos, glyph) in entities.iter() {
        if map.visible.get_bit(pos.0) {
            let entity_screen_pos = camera.screen_to_world(pos.0);
            if map.in_bounds(Point::new(entity_screen_pos.x, entity_screen_pos.y)) {
                batch.set(entity_screen_pos, glyph.color, glyph.glyph);
            }
        }
    }

    batch.submit(BATCH_CHARS).expect("Error batching map");
}
