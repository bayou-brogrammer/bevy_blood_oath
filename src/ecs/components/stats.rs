use crate::prelude::*;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl CombatStats {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32) -> Self {
        assert!(max_hp > 0 && hp > 0 && hp <= max_hp);

        CombatStats { max_hp, hp, defense, power }
    }
}
