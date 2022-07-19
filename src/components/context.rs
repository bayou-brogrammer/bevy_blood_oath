use super::*;

#[derive(Debug)]
pub struct Mouse {
    pub left_click: bool,
    pub mouse_pos: (i32, i32),
}

impl Mouse {
    pub fn new(mouse_pos: (i32, i32), left_click: bool) -> Self {
        Self {
            left_click,
            mouse_pos,
        }
    }
}
