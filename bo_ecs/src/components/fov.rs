use crate::prelude::*;
use bracket_geometry::prelude::Point;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfView {
    pub radius: i32,
    pub is_dirty: bool,
    pub visible_tiles: HashSet<Point>,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        assert!(radius >= 0);

        Self { radius, is_dirty: true, visible_tiles: HashSet::new() }
    }
}
