use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub pt: Point,
    pub layer: usize,
}

impl Position {
    pub fn with_pt(pt: Point, layer: usize) -> Self {
        Self { pt, layer }
    }
}
