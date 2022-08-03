use crate::prelude::*;
use std::collections::HashMap;

mod loading;
mod saving;
pub use loading::*;
pub use saving::*;

pub type BoxedError = Box<dyn std::error::Error>;

// Special component that exists to help serialize the game data
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: Map,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct DMSerializationHelper {
    pub map: MasterDungeonMap,
    pub log: Vec<Vec<LogFragment>>,
    pub events: HashMap<String, i32>,
}

impl_new!(SerializationHelper, map: Map);
impl_new!(
    DMSerializationHelper,
    map: MasterDungeonMap,
    log: Vec<Vec<LogFragment>>,
    events: HashMap<String, i32>
);
