use bevy_ecs::{
    schedule::{ShouldRun, StateData},
    system::Res,
};
use iyes_loopless::state::CurrentState;

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
