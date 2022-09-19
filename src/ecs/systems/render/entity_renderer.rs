use crate::prelude::*;

pub fn entity_render(
    (map, camera): (Res<Map>, Res<CameraView>),
    glyph_q: Query<(&Point, &Glyph), (Without<Hidden>, Without<ParticleLifetime>, Without<Prop>)>,
) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_CHAR);

    let mut entities = glyph_q.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

    for (pos, glyph) in entities.iter() {
        if map.visible.get_bit(**pos) {
            let entity_screen_pos = camera.world_to_screen(**pos);
            batch.set(entity_screen_pos, glyph.color, glyph.glyph);
        }
    }

    batch.submit(BATCH_CHARS).expect("Error batching map");
}

pub fn particle_render(
    (map, camera): (Res<Map>, Res<CameraView>),
    glyph_q: Query<(&Point, &Glyph), With<ParticleLifetime>>,
) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_PARTICLE);

    for (pos, glyph) in glyph_q.iter() {
        if map.visible.get_bit(*pos) {
            let entity_screen_pos = camera.world_to_screen(*pos);
            batch.set(entity_screen_pos, glyph.color, glyph.glyph);
        }
    }

    batch.submit(BATCH_CHARS).expect("Error batching map");
}
