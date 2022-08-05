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
        VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => Some(GameKey::Up),
        VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => Some(GameKey::Down),
        VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => Some(GameKey::Left),
        VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => Some(GameKey::Right),
        VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => Some(GameKey::LeftUp),
        VirtualKeyCode::U | VirtualKeyCode::Numpad9 => Some(GameKey::RightUp),
        VirtualKeyCode::B | VirtualKeyCode::Numpad1 => Some(GameKey::LeftDown),
        VirtualKeyCode::N | VirtualKeyCode::Numpad3 => Some(GameKey::RightDown),

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
