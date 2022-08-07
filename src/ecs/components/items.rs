use crate::prelude::*;

#[derive(Component, Debug)]
pub struct ProvidesHealing(pub i32);

#[derive(Component, Debug)]
pub struct InflictsDamage(pub i32);

#[derive(Component, Debug)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

impl_new!(Confusion, turns: i32);
