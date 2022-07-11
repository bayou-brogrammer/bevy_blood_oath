#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColonistStatus {
    Alive,
    StartedDead,
    DiedAfterStart,
    Rescued,
}
