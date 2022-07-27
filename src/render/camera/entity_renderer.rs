use super::*;

pub fn entity_render(
    map: Res<Map>,
    camera: Res<GameCamera>,
    glyph_q: Query<(&Position, &Glyph), Without<ParticleLifetime>>,
) {
    let mut batch = DrawBatch::new();
    batch.target(LAYER_MAP);

    let mut entities = glyph_q.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

    for (pos, glyph) in entities.iter() {
        if map.visible.get_bit(pos.0) {
            let entity_screen_pos = camera.screen_to_world(pos.0);
            if map.in_bounds(Point::new(entity_screen_pos.x, entity_screen_pos.y)) {
                batch.set(
                    Point::new(entity_screen_pos.x, entity_screen_pos.y),
                    glyph.color,
                    glyph.glyph,
                );
            }
        }
    }

    batch.submit(BATCH_CHARS).expect("Error batching map");
}

// pub fn particle_render(
//     map: Res<Map>,
//     camera: Res<GameCamera>,
//     particles_q: Query<(&Position, &Glyph), (With<ParticleLifetime>, Without<Item>)>,
// ) {
//     let mut batch = DrawBatch::new();
//     batch.target(LAYER_PARTICLES);

//     for (pos, glyph) in particles_q.iter() {
//         if map.visible.get_bit(pos.0) {
//             let screen_pos = camera.world_to_screen(pos.0);
//             batch.set(screen_pos, glyph.color, glyph.glyph);
//         }
//     }

//     batch.submit(1000000).expect("Error batching particles");
// }
