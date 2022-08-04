use crate::prelude::*;

pub trait VirtualGameKey {
    fn get_key(&self) -> Option<GameKey>;
}

#[derive(PartialEq, Debug)]
pub enum GameKey {
    // Movement
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    // Actions
    Escape,
    Select,
    SkipTurn,
    Pickup,
    Inventory,
    Drop,
    Remove,
}

fn key_mapping(key: VirtualKeyCode) -> Option<GameKey> {
    match key {
        // Movement
        VirtualKeyCode::Up => Some(GameKey::Up),
        VirtualKeyCode::Down => Some(GameKey::Down),
        VirtualKeyCode::Left => Some(GameKey::Left),
        VirtualKeyCode::Right => Some(GameKey::Right),
        VirtualKeyCode::Y => Some(GameKey::LeftUp),
        VirtualKeyCode::U => Some(GameKey::RightUp),
        VirtualKeyCode::B => Some(GameKey::LeftDown),
        VirtualKeyCode::N => Some(GameKey::RightDown),

        // Actions
        VirtualKeyCode::Escape => Some(GameKey::Escape),
        VirtualKeyCode::Return => Some(GameKey::Select),
        VirtualKeyCode::Space => Some(GameKey::SkipTurn),
        VirtualKeyCode::G => Some(GameKey::Pickup),
        VirtualKeyCode::I => Some(GameKey::Inventory),
        VirtualKeyCode::D => Some(GameKey::Drop),
        VirtualKeyCode::R => Some(GameKey::Remove),
        _ => None,
    }
}

impl VirtualGameKey for VirtualKeyCode {
    fn get_key(&self) -> Option<GameKey> {
        key_mapping(*self)
    }
}

impl VirtualGameKey for Option<&VirtualKeyCode> {
    fn get_key(&self) -> Option<GameKey> {
        if let Some(key) = self.as_deref() {
            key_mapping(*key)
        } else {
            None
        }
    }
}
