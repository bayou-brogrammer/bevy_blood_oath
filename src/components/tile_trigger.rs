use crate::prelude::*;

#[derive(Debug)]
pub enum TriggerType {
    EndGame,
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct TileTrigger(pub TriggerType);
