use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

impl_new!(InBackpack, owner: Entity);
