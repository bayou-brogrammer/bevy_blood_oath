use crate::prelude::*;
use std::cmp::{max, min};

const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 43;
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

mod tile;
use tile::{Tile, TileType};

#[derive(Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
    pub starting_point: Point,
}

impl Map {
    fn apply_room_to_map(&mut self, room: &Rect) {
        room.for_each(|pt| {
            let idx = self.point2d_to_index(pt);
            self.tiles[idx] = Tile::floor();
        });
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.point2d_to_index(Point::new(x, y));
            if self.tiles[idx as usize].tile_type == TileType::Wall {
                self.tiles[idx as usize] = Tile::floor();
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.point2d_to_index(Point::new(x, y));
            if self.tiles[idx as usize].tile_type == TileType::Wall {
                self.tiles[idx as usize] = Tile::floor();
            }
        }
    }

    pub fn clear_content_index(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.contents.clear();
        }
    }

    pub fn populate_blocked(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.blocked = tile.tile_type == TileType::Wall;
        }
    }

    pub fn clear_visible(&mut self) {
        self.visible.iter_mut().for_each(|b| *b = false);
    }

    pub fn set_visibility(&mut self, point: Point) {
        if self.in_bounds(point) {
            let idx = self.point2d_to_index(point);
            self.visible[idx] = true;
            self.revealed[idx] = true;
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && !self.tiles[self.point2d_to_index(point)].blocked
    }

    pub fn update_blocked(&mut self, old_pt: Point, new_pt: Point) {
        let old_idx = self.point2d_to_index(old_pt);
        let new_idx = self.point2d_to_index(new_pt);
        self.tiles[old_idx].blocked = false;
        self.tiles[new_idx].blocked = true;
    }

    pub fn new_map_rooms_and_corridors() -> Self {
        let mut map = Self {
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            visible: vec![false; MAPCOUNT],
            revealed: vec![false; MAPCOUNT],
            tiles: vec![Tile::wall(); MAPCOUNT],
            starting_point: Point::new(MAPWIDTH / 2, MAPHEIGHT / 2),
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = crate::rng::RNG.write();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let ok = map.rooms.iter().all(|room| !new_room.intersect(room));

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point {
                        x: prev_x,
                        y: prev_y,
                    } = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                } else {
                    map.starting_point = new_room.center();
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    // Private
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y > 0 && pos.y < self.height as i32
    }
}

#[rustfmt::skip]
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].opaque
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        // Cardinals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) { exits.push((idx, 1.0)) }

        // Diagonals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) { exits.push((idx, 1.45)) }

        exits
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type

        if map.revealed[idx] {
            let tile = &tile;
            let tint = if map.visible[idx] { GREEN } else { DARK_GRAY };
            let color = ColorPair::new(tint, tile.color.bg);

            ctx.set(x, y, color.fg, color.bg, tile.glyph);
        }

        // Move the coordinates
        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}
