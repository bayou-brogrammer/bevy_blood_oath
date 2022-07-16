use super::*;

#[derive(Debug)]
pub struct Mouse {
    pub left_click: bool,
    pub mouse_pos: Point,
}

impl Mouse {
    pub fn new(mouse_pos: Point, left_click: bool) -> Self {
        Self {
            left_click,
            mouse_pos,
        }
    }
}
