use crate::prelude::*;
use bracket_geometry::prelude::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Position(pub Point);

impl Position {
    pub fn new(pt: Point) -> Self {
        Self(pt)
    }

    pub fn new_xy(x: i32, y: i32) -> Self {
        Self(Point::new(x, y))
    }
}
