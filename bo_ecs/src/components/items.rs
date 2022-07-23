use crate::prelude::*;

#[derive(Component, Debug)]
pub struct ProvidesHealing(pub i32);

#[derive(Component, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug)]
pub struct Confusion {
    pub turns: i32,
}
