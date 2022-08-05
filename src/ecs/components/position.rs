use crate::prelude::*;

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

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct OtherLevelPosition {
    pub pt: Point,
    pub depth: i32,
}

impl_new!(OtherLevelPosition, pt: Point, depth: i32);
