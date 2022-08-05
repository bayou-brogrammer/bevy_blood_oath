#![allow(clippy::module_inception)]

use crate::prelude::*;
use std::collections::{HashMap, HashSet};

pub mod map_builders;
pub mod spatial;

mod bitgrid;
mod dungeon;
mod themes;
mod tiletype;

pub use bitgrid::*;
pub use dungeon::*;
pub use map_builders::{BuilderMap, MapGenTimer};
pub use themes::*;
pub use tiletype::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub name: String,
    pub visible: BitGrid,
    pub revealed: BitGrid,
    pub tiles: Vec<GameTile>,
    pub view_blocked: HashSet<usize>,
    pub bloodstains: HashMap<usize, RGB>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn get_tile_type(&self, tt: TileType) -> Vec<GameTile> {
        self.tiles.iter().filter(|t| t.tile_type == tt).cloned().collect::<Vec<_>>()
    }

    pub fn clear_content_index(&mut self) {
        crate::spatial::clear();
    }

    pub fn populate_blocked(&mut self) {
        crate::spatial::populate_blocked_from_map(self);
    }

    pub fn clear_visible(&mut self) {
        self.visible.zero_out_bits();
    }

    pub fn set_revealed_and_visible(&mut self, pt: Point) {
        if self.in_bounds(pt) {
            self.visible.set_bit(pt, true);
            self.revealed.set_bit(pt, true);
        }
    }

    pub fn can_enter_tile(&self, pt: Point) -> bool {
        let idx = self.point2d_to_index(pt);
        self.in_bounds(pt) && !crate::spatial::is_blocked(idx)
    }

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> Map {
        let map_tile_count = (width * height) as usize;
        crate::spatial::set_size(width, height);

        Map {
            width,
            height,
            depth: new_depth,
            name: name.to_string(),
            bloodstains: HashMap::new(),
            view_blocked: HashSet::new(),
            visible: BitGrid::new(width, height),
            revealed: BitGrid::new(width, height),
            tiles: vec![GameTile::wall(); map_tile_count],
        }
    }

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
        self.tiles[idx].opaque || self.view_blocked.contains(&idx)
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

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}
