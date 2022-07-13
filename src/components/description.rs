use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
#[storage(VecStorage)]
pub struct Description(pub String);
