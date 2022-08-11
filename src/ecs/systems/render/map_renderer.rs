use crate::prelude::*;

pub fn map_render(camera: Res<CameraView>, map: Res<Map>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_ZERO);

    camera.viewport.for_each(|pt| {
        let screen_pt = camera.world_to_screen(pt);
        if map.in_bounds(pt) {
            let idx = map.point2d_to_index(pt);
            if map.revealed.get_bit(pt) {
                let (glyph, color) = map.tile_glyph(idx);
                draw_batch.set(screen_pt, color, glyph);
            }
        } else if SHOW_BOUNDARIES {
            draw_batch.set(screen_pt, ColorPair::new(GRAY, BLACK), to_cp437('·'));
        }
    });

    draw_batch.submit(BATCH_ZERO).expect("Error batching map");
}
