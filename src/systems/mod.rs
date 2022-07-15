use crate::prelude::*;

pub mod dispatcher;
pub use dispatcher::*;

pub mod fov_system;
pub use fov_system::FovSystem;

mod map_indexing_system;
pub use map_indexing_system::MapIndexingSystem;

pub mod damage_system;
pub use damage_system::DamageSystem;

mod melee_combat_system;
pub use melee_combat_system::MeleeCombatSystem;

mod monster_ai_system;
pub use monster_ai_system::MonsterAISystem;

pub fn new_ticking_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!(
        (FovSystem, "fov", &[]),
        (MonsterAISystem, "ai_system", &[]),
        (MapIndexingSystem, "map_indexing", &[]),
        (MeleeCombatSystem, "melee_combat", &[]),
        (DamageSystem, "damage", &[])
    );
}
