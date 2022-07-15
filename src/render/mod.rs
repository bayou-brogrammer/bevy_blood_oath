use crate::prelude::*;

mod camera;

pub fn clear_all_consoles(ctx: &mut BTerm) {
    for layer in 0..5 {
        ctx.set_active_console(layer);
        ctx.cls();
    }
    ctx.set_active_console(0);
}

pub fn render_gui(ctx: &mut BTerm, world: &mut World) {
    render_ui(ctx, world);

    let camera = camera::Camera::new(world);
    let map = world.fetch::<Map>();

    camera.render_map(&map);
    camera.render_glyphs(&map, world);
}

pub fn render_ui(ctx: &mut BTerm, ecs: &World) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, RGB::named(YELLOW), RGB::named(BLACK), &health);

        ctx.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(RED),
            RGB::named(BLACK),
        );
    }

    crate::gamelog::print_log(
        &mut BACKEND_INTERNAL.lock().consoles[0].console,
        // &mut BACKEND_INTERNAL.lock().consoles[1].console,
        Point::new(1, 44),
    );
}
