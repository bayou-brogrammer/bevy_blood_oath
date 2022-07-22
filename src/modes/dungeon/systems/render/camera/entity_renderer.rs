use super::*;

pub fn entity_render(
    map: Res<Map>,
    camera: Res<GameCamera>,
    glyph_q: Query<(&Position, &Glyph), (Without<Item>, Without<ParticleLifetime>)>,
) {
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

    batch.submit(BATCH_CHARS).expect("Error batching map");
}

pub fn item_render(
    map: Res<Map>,
    camera: Res<GameCamera>,
    items_q: Query<(&Position, &Glyph), (With<Item>, Without<ParticleLifetime>)>,
) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_ITEMS);

    for (pos, glyph) in items_q.iter() {
        if map.visible.get_bit(pos.0) {
            let screen_pos = camera.world_to_screen(pos.0);
            batch.set(screen_pos, glyph.color, glyph.glyph);
        }
    }

    batch.submit(BATCH_ITEMS).expect("Error batching map");
}

pub fn particle_render(
    map: Res<Map>,
    camera: Res<GameCamera>,
    particles_q: Query<(&Position, &Glyph), (With<ParticleLifetime>, Without<Item>)>,
) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_PARTICLES);

    for (pos, glyph) in particles_q.iter() {
        if map.visible.get_bit(pos.0) {
            let screen_pos = camera.world_to_screen(pos.0);
            batch.set(screen_pos, glyph.color, glyph.glyph);
        }
    }

    batch.submit(1000000).expect("Error batching particles");
}
