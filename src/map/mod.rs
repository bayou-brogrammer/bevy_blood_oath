#![allow(clippy::module_inception)]

use crate::prelude::*;
use std::collections::HashMap;

pub mod map_builders;
pub mod spatial;

mod bitgrid;
mod dungeon;
mod themes;
mod tiletype;

pub use bitgrid::*;
pub use dungeon::*;
pub use map_builders::BuilderMap;
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
        crate::spatial::clear_content_index();
        crate::spatial::clear_blocked();
        crate::spatial::clear_opaque();
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
            visible: BitGrid::new(width, height),
            revealed: BitGrid::new(width, height),
            tiles: vec![GameTile::wall(); map_tile_count],
        }
    }

    pub fn tile_glyph(&self, idx: usize) -> (FontCharType, ColorPair) {
        let tile = &self.tiles[idx];
        let (glyph, mut color) = match self.depth {
            3 => tile.get_limestone_glyph(),
            2 => tile.get_forest_glyph(),
            _ => tile.get_tile_glyph_default(self, idx),
        };

        if self.bloodstains.contains_key(&idx) {
            color.bg = (*self.bloodstains.get(&idx).unwrap()).into();
        }

        if !self.visible.get_bit(self.index_to_point2d(idx)) {
            color.fg = color.fg.to_greyscale();
            color.bg = RGBA::from_f32(0., 0., 0., 0.); // Don't show stains out of visual range
        }

        (glyph, color)
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
    fn is_opaque(&self, idx:usize) -> bool {
        if idx > 0 && idx < self.tiles.len() {
            crate::spatial::is_opaque(idx)
        } else {
            true
        }
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        let tt = self.tiles[idx];


        // Cardinals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) { exits.push((idx, tt.cost)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) { exits.push((idx, tt.cost)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) { exits.push((idx, tt.cost)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) { exits.push((idx, tt.cost)) }

        // Diagonals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) { exits.push((idx, tt.cost)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) { exits.push((idx, tt.cost)) }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) { exits.push((idx, tt.cost)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) { exits.push((idx, tt.cost)) }

        exits
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}
