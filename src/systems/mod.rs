use crate::prelude::*;

pub mod dispatcher;
pub use dispatcher::*;

pub mod fov_system;
use fov_system::FovSystem;

mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;

pub mod damage_system;
use damage_system::DamageSystem;

pub fn new_ticking_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!(
        (FovSystem, "fov", &[]),
        (MapIndexingSystem, "map_indexing", &[]),
        (DamageSystem, "damage", &[])
    );
}
