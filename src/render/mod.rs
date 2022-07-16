use self::gui::LOG_PANEL_BOX;
use crate::prelude::*;

pub mod camera;
mod gui;
use gui::safe_print_color;

pub fn clear_all_consoles(ctx: &mut BTerm) {
    for layer in 0..LAYERS {
        ctx.set_active_console(layer);
        ctx.cls();
    }
    ctx.set_active_console(0);
}

pub fn render_camera(player_entity: Entity, world: &mut World) {
    render_ui(world);

    let camera = camera::Camera::new(player_entity, world);
    world.resource_scope(|world, map: Mut<Map>| {
        camera.render_map(&map);
        camera.render_glyphs(&map, world);
        camera.render_tooltips(&map, world);
    });
}

pub fn render_ui(world: &mut World) {
    let mut gui_batch = DrawBatch::new();

    gui::render_panels(&mut gui_batch);
    gui::render_status(&mut gui_batch, world);
    gamelog::print_log(&mut gui_batch, Point::new(1, LOG_PANEL_BOX.y1 + 1));

    gui_batch.submit(50_000).expect("Batch error"); // On top of everything
}
