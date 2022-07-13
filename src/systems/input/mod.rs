use super::*;

mod player_input_system;
use player_input_system::PlayerInputSystem;

mod exit_system;
use exit_system::ExitSystem;

pub fn new_input_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!(
        (PlayerInputSystem, "player_input", &[]),
        (ExitSystem, "exit", &[])
    );
}
