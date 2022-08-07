use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HungerState {
    WellFed,
    Normal,
    Hungry,
    Starving,
}

#[derive(Component, Clone)]
pub struct HungerClock {
    pub duration: i32,
    pub state: HungerState,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProvidesFood {}

impl_new!(HungerClock, state: HungerState, duration: i32);
