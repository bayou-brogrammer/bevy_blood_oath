use self::gui::LOG_PANEL_BOX;
use crate::prelude::*;

pub mod camera;
pub mod gui;

pub fn clear_all_consoles(ctx: &mut BTerm, consoles: &Vec<usize>) {
    for layer in consoles.iter() {
        ctx.set_active_console(*layer);
        ctx.cls();
    }

    if !consoles.is_empty() {
        ctx.set_active_console(consoles[0])
    }
}

pub fn render_camera(world: &mut World) {
    let camera = camera::Camera::new(world);

    world.resource_scope(|world, map: Mut<Map>| {
        camera.render_map(&map);
        camera.render_glyphs(&map, world);
        camera.render_tooltips(&map, world);
    });
}

fn render_ui(stats_q: Query<&CombatStats, With<Player>>) {
    let mut gui_batch = DrawBatch::new();

    gui::render_panels(&mut gui_batch);
    gui::render_status(&mut gui_batch, stats_q);
    gamelog::print_log(&mut gui_batch, Point::new(1, LOG_PANEL_BOX.y1 + 1));

    gui_batch.submit(40_000).expect("Batch error"); // On top of everything
}

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(GameStage::Render, render_camera.exclusive_system())
            .add_plugin(GUIPlugin);
    }
}

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::Render,
            ConditionSet::new().with_system(render_ui).into(),
        );
    }
}
