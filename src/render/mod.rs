use crate::prelude::*;

mod gui;
use gui::*;

mod camera;

pub fn clear_all_consoles(ctx: &mut BTerm) {
    for layer in 0..5 {
        ctx.set_active_console(layer);
        ctx.cls();
    }
    ctx.set_active_console(0);
}

pub fn new_gui_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!();
}

pub fn render(world: &World) {
    let camera = camera::Camera::new(world);

    let mut gui_batch = DrawBatch::new();
    gui::render_panels(&mut gui_batch);
    // gui::render_status(&mut gui_batch, &status);
    gui_batch.submit(50_000).expect("Batch error"); // On top of everything

    let map = world.fetch::<Map>();
    camera.render_map(&map);
    camera.render_glyphs(&map, world);
}

// pub fn render_gui(ecs: &mut World, resources: &mut Resources) {
//     let map = resources.get::<Map>().unwrap();
//     let status = gui::PlayerStatus::query(ecs, map.current_layer);
//     let camera = camera::Camera::new(ecs);
//     let mouse = resources.get::<Mouse>().unwrap();

//     let mut gui_batch = DrawBatch::new();
// gui::render_panels(&mut gui_batch);
// gui::render_status(&mut gui_batch, &status);
// gui::render_colony_info(&mut gui_batch, &status.colony);
// // gui::render_targeting(&mut gui_batch, &status.target);
//     gui_batch.submit(50_000).expect("Batch error"); // On top of everything

//     camera.render_map(&map);
//     camera.render_glyphs(&map, ecs);
//     // camera.render_speech(ecs, map);
//     // camera.render_projectiles(ecs, map);
//     // camera.render_targeting(&status.target);
//     camera.render_tooltips(ecs, &map, &mouse);
// }

pub struct RenderGlyphSystem {}

impl<'a> System<'a> for RenderGlyphSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Glyph>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, pos_storage, glyph_storage) = data;

        let mut batch = DrawBatch::new();
        batch.target(LAYER_CHR);

        for (pos, glyph) in (&pos_storage, &glyph_storage).join() {
            if pos.layer == map.current_layer {
                let idx = map.get_current().point2d_to_index(pos.pt);
                if map.get_current().visible[idx] {
                    let screen_pos = self.world_to_screen(pos.pt);
                    batch.set(screen_pos, glyph.color, glyph.glyph);
                }
            }
        }

        batch.submit(4000).expect("Error batching map");
    }
}
