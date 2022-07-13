use super::{layerbuilder::build_entrance, *};

#[derive(Clone)]
pub struct Layer {
    pub tiles: Vec<Tile>,
    pub is_door: Vec<bool>,
    pub revealed: Vec<bool>,
    pub visible: Vec<bool>,
    pub starting_point: Point,
}

impl Layer {
    pub fn new(depth: usize, ecs: &mut World) -> Self {
        match depth {
            0 => build_entrance(ecs),
            _ => Self {
                is_door: vec![false; TILES],
                visible: vec![false; TILES],
                revealed: vec![false; TILES],
                tiles: vec![Tile::default(); TILES],
                starting_point: Point::new(WIDTH / 2, HEIGHT / 2),
            },
        }
    }

    pub fn clear_visible(&mut self) {
        self.visible.iter_mut().for_each(|b| *b = false);
    }

    pub fn create_door(&mut self, idx: usize) {
        self.tiles[idx] = Tile::wall();
        self.tiles[idx].glyph = to_cp437('+');
        self.tiles[idx].color.fg = CYAN.into();
        self.is_door[idx] = true;
    }

    pub fn open_door(&mut self, idx: usize) {
        self.is_door[idx] = false;
        self.tiles[idx].blocked = false;
        self.tiles[idx].opaque = false;
        self.tiles[idx].glyph = to_cp437('.');
    }

    // Private
    fn test_exit(&self, pt: Point, delta: Point, exits: &mut SmallVec<[(usize, f32); 10]>) {
        let dest_pt = pt + delta;
        if self.in_bounds(dest_pt) {
            let dest_idx = self.point2d_to_index(pt + delta);
            if !self.tiles[dest_idx].blocked {
                exits.push((dest_idx, 1.0));
            }
        }
    }
}

impl Algorithm2D for Layer {
    fn dimensions(&self) -> Point {
        Point::new(WIDTH, HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < WIDTH as i32 && pos.y > 0 && pos.y < HEIGHT as i32
    }
}

impl BaseMap for Layer {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].opaque
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let pt = self.index_to_point2d(idx);
        self.test_exit(pt, Point::new(-1, 0), &mut exits);
        self.test_exit(pt, Point::new(1, 0), &mut exits);
        self.test_exit(pt, Point::new(0, -1), &mut exits);
        self.test_exit(pt, Point::new(0, 1), &mut exits);
        exits
    }
}
