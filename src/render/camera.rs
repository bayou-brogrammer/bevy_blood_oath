use super::*;

pub struct Camera {
    player_pos: Point,
    viewport: Rect,
}

impl Camera {
    pub fn new(world: &World) -> Self {
        let player_pos = *world.read_resource::<Point>();
        let viewport = Rect::with_size(player_pos.x - 20, player_pos.y - 15, 40, 31);

        Self {
            player_pos,
            viewport,
        }
    }

    fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos;
        bot + Point::new(20, 15)
    }

    fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    fn screen_to_world(&self, mouse_x: i32, mouse_y: i32) -> Point {
        Point::new(mouse_x + self.viewport.x1, mouse_y + self.viewport.y1)
    }

    pub fn render_map(&self, map: &Map) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_MAP);

        self.viewport.for_each(|pt| {
            let idx = map.point2d_to_index(pt);
            if map.in_bounds(pt) && map.revealed[idx] {
                let t = &map.tiles[idx];
                let tint = if map.visible[idx] { GREEN } else { DARK_GRAY };
                let color = ColorPair::new(tint, t.color.bg);

                batch.set(self.world_to_screen(pt), color, t.glyph);
            }
        });

        batch.submit(0).expect("Error batching map");
    }

    pub fn render_glyphs(&self, map: &Map, world: &World) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_CHR);

        let positions = world.read_storage::<Position>();
        let glyphs = world.read_storage::<Glyph>();

        for (pos, glyph) in (&positions, &glyphs).join() {
            let idx = map.point2d_to_index(pos.0);
            if map.visible[idx] {
                let screen_pos = self.world_to_screen(pos.0);
                batch.set(screen_pos, glyph.color, glyph.glyph);
            }
        }

        batch.submit(4000).expect("Error batching map");
    }
}
