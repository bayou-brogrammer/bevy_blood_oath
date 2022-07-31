use crate::prelude::*;
use bo_utils::impl_new;
use bracket_terminal::prelude::RGB;

#[derive(Component, Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}

#[derive(Component, Clone)]
pub struct DefenseBonus {
    pub defense: i32,
}

#[derive(Component, Clone, Debug)]
pub struct Blood(pub RGB);

impl_new!(DefenseBonus, defense: i32);
impl_new!(MeleePowerBonus, power: i32);
