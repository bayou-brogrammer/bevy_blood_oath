use crate::prelude::*;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct OtherLevelPosition {
    pub pt: Point,
    pub depth: i32,
}

impl_new!(OtherLevelPosition, pt: Point, depth: i32);
