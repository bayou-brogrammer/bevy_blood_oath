use super::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
#[storage(DenseVecStorage)]
pub struct FieldOfView {
    pub radius: i32,
    pub is_dirty: bool,
    pub visible_tiles: HashSet<Point>,
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
