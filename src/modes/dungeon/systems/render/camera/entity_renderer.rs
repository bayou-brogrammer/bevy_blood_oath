use super::*;

pub fn entity_render(camera: Res<GameCamera>, map: Res<Map>, glyph_q: Query<(&Position, &Glyph)>) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_CHARS);

    let mut entities = glyph_q.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

    for (pos, glyph) in entities.iter() {
        if map.visible.get_bit(pos.0) {
            let screen_pos = camera.world_to_screen(pos.0);
            batch.set(screen_pos, glyph.color, glyph.glyph);
        }
    }

    batch.submit(4000).expect("Error batching map");
}
