use crate::prelude::*;

#[derive(Debug)]
pub struct Mouse {
    pub left_click: bool,
    pub mouse_pos: Point,
}

#[derive(Debug, Default)]
pub struct Key(pub Option<VirtualKeyCode>);

#[derive(Debug, Default)]
pub struct Quit(pub bool);
