use super::{gui::safe_print_color, *};
use bracket_lib::prelude::Rect;

pub struct Camera {
    player_pos: Point,
    viewport: Rect,
}

impl Camera {
    pub fn new(world: &mut World) -> Self {
        let player_pos = *world.resource::<Point>();
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
            if map.in_bounds(pt) && map.revealed.get_bit(pt) {
                let t = &map.tiles[idx];

                let tint = if map.visible.get_bit(pt) {
                    GREEN
                } else {
                    DARK_GRAY
                };

                let color = ColorPair::new(tint, t.color.bg);
                batch.set(self.world_to_screen(pt), color, t.glyph);
            }
        });

        batch.submit(0).expect("Error batching map");
    }

    pub fn render_glyphs(&self, map: &Map, world: &mut World) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_CHARS);

        let mut query = world.query::<(&Position, &Glyph)>();
        let mut entities = query.iter(&world).collect::<Vec<_>>();
        entities.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

        for (pos, glyph) in entities.iter() {
            if map.visible.get_bit(pos.0) {
                let screen_pos = self.world_to_screen(pos.0);
                batch.set(screen_pos, glyph.color, glyph.glyph);
            }
        }

        batch.submit(4000).expect("Error batching map");
    }

    pub fn render_tooltips(&self, map: &Map, world: &mut World) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_TEXT);

        let (mouse_x, mouse_y) = world.resource::<Mouse>().mouse_pos;
        let map_pos = self.screen_to_world(mouse_x, mouse_y);

        let mut lines = Vec::new();
        let mut query = world.query::<(
            &Position,
            &Naming,
            Option<&Description>,
            Option<&CombatStats>,
        )>();

        query
            .iter(world)
            .filter(|(pos, _, _, _)| pos.0 == map_pos)
            .for_each(|(pos, name, desc, stats)| {
                if map.visible.get_bit(pos.0) {
                    lines.push((CYAN, name.0.clone()));

                    if let Some(desc) = desc {
                        lines.push((GRAY, desc.0.clone()));
                    }

                    if let Some(stats) = stats {
                        lines.push((GRAY, format!("{}/{} hp", stats.hp, stats.max_hp)));
                    }
                }
            });

        if !lines.is_empty() {
            let height = lines.len() + 1;
            let width = lines.iter().map(|s| s.1.len()).max().unwrap() + 2;
            let tip_x = if map_pos.x < MAPWIDTH as i32 / 2 {
                i32::min((mouse_x * 2) + 1, 111)
            } else {
                i32::max(0, (mouse_x * 2) - (width as i32 + 1))
            };
            let tip_y = if map_pos.y > MAPHEIGHT as i32 / 2 {
                mouse_y - height as i32
            } else {
                mouse_y
            };

            batch.draw_box(
                Rect::with_size(
                    tip_x,
                    tip_y - (lines.len() / 2) as i32,
                    width as i32,
                    height as i32,
                ),
                ColorPair::new(WHITE, BLACK),
            );

            let mut y = tip_y + 1 - (lines.len() / 2) as i32;
            lines.iter().for_each(|s| {
                safe_print_color(
                    &mut batch,
                    Point::new(tip_x + 1, y),
                    &s.1,
                    ColorPair::new(s.0, BLACK),
                );
                y += 1;
            });
        }

        batch.submit(100_000).expect("Error batching tooltips");
    }
}
