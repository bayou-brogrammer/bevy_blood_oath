use super::*;

#[derive(Debug)]
pub struct Camera {
    player_pos: Point,
    viewport: Rect,
}

impl Camera {
    pub fn new(world: &World) -> Self {
        let positions = world.read_component::<Position>();
        let player = world.fetch::<Player>();
        let player_pos = positions.get(player.id).unwrap().pt;

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

        let layer = map.get_current();
        self.viewport.for_each(|pt| {
            let idx = layer.point2d_to_index(pt);
            if layer.in_bounds(pt) && layer.revealed[idx] {
                let t = &layer.tiles[idx];
                let mut color = t.color;

                if !layer.visible[idx] {
                    color.fg = color.fg.to_greyscale();
                }

                batch.set(self.world_to_screen(pt), t.color, t.glyph);
            }
        });

        batch.submit(0).expect("Error batching map");
    }

    pub fn render_glyphs(&self, map: &Map, world: &World) {
        let mut batch = DrawBatch::new();
        batch.target(LAYER_CHR);

        let positions = world.read_component::<Position>();
        let glyphs = world.read_component::<Glyph>();

        for (pos, glyph) in (&positions, &glyphs).join() {
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

    // pub fn render_tooltips(&self, ecs: &mut World, map: &Map, mouse: &Mouse) {
    //     let mut batch = DrawBatch::new();
    //     batch.target(LAYER_TEXT);

    //     let Point {
    //         x: mouse_x,
    //         y: mouse_y,
    //     } = mouse.mouse_pos;
    //     let map_pos = self.screen_to_world(mouse_x, mouse_y);
    //     let mut new_target = None;

    //     let mut lines = Vec::new();
    //     <(Entity, &Position, &Description, &Name)>::query().for_each(
    //         ecs,
    //         |(entity, pos, desc, name)| {
    //             if pos.layer == map.current_layer && pos.pt == map_pos {
    //                 let idx = map.get_current().point2d_to_index(pos.pt);
    //                 if map.get_current().visible[idx] {
    //                     lines.push((CYAN, name.0.clone()));
    //                     lines.push((GRAY, desc.0.clone()));
    //                     // if let Ok(er) = ecs.entry_ref(*entity) {
    //                     //     if let Ok(hp) = er.get_component::<Health>() {
    //                     //         lines.push((GRAY, format!("{}/{} hp", hp.current, hp.max)));
    //                     //     }
    //                     // }
    //                     if mouse.left_click {
    //                         //println!("Set new target");
    //                         new_target = Some(*entity);
    //                     }
    //                 }
    //             }
    //         },
    //     );

    //     if !lines.is_empty() {
    //         let height = lines.len() + 1;
    //         let width = lines.iter().map(|s| s.1.len()).max().unwrap() + 2;
    //         let tip_x = if map_pos.x < WIDTH as i32 / 2 {
    //             i32::min((mouse_x * 2) + 1, 111)
    //         } else {
    //             i32::max(0, (mouse_x * 2) - (width as i32 + 1))
    //         };
    //         let tip_y = if map_pos.y > HEIGHT as i32 / 2 {
    //             mouse_y - height as i32
    //         } else {
    //             mouse_y
    //         };
    //         batch.draw_box(
    //             Rect::with_size(
    //                 tip_x,
    //                 tip_y - (lines.len() / 2) as i32,
    //                 width as i32,
    //                 height as i32,
    //             ),
    //             ColorPair::new(WHITE, BLACK),
    //         );
    //         let mut y = tip_y + 1 - (lines.len() / 2) as i32;
    //         lines.iter().for_each(|s| {
    //             safe_print_color(
    //                 &mut batch,
    //                 Point::new(tip_x + 1, y),
    //                 &s.1,
    //                 ColorPair::new(s.0, BLACK),
    //             );
    //             y += 1;
    //         });
    //     }

    //     batch.submit(100_000).expect("Error batching tooltips");
    // }
}
