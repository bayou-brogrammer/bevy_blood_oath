use crate::prelude::*;
use bevy::ecs::schedule::{ShouldRun, StateData};

////////////////////////////////////////////////////////////////////////////////
/// Run Criteria
////////////////////////////////////////////////////////////////////////////////

pub fn run_in_state_bevy<T: StateData>(
    state: T,
) -> impl Fn(Res<CurrentState<T>>) -> ShouldRun + Clone + 'static {
    move |current: Res<CurrentState<T>>| -> ShouldRun {
        if current.0 == state {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
