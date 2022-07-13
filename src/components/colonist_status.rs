use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
#[storage(VecStorage)]
pub enum ColonistStatus {
    Alive,
    StartedDead,
    DiedAfterStart,
    Rescued,
}
