use bracket_lib::prelude::Point;
use std::collections::HashSet;

pub struct FieldOfView {
    pub radius: i32,
    pub visible_tiles: HashSet<Point>,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            radius,
            is_dirty: true,
            visible_tiles: HashSet::new(),
        }
    }
}
