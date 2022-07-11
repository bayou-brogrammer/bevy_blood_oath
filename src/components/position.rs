use bracket_lib::prelude::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub pt: Point,
    pub layer: usize,
}

impl Position {
    pub fn with_pt(pt: Point, layer: usize) -> Self {
        Self { pt, layer }
    }
}
