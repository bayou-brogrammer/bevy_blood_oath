mod components;
mod effects;
mod events;
mod resources;

pub mod queries;

pub mod prelude {
    pub use crate::queries;
    pub use bevy_ecs::prelude::{Bundle, Component, Entity};

    pub use crate::components::*;
    pub use crate::effects::*;
    pub use crate::events::*;
    pub use crate::queries::*;
    pub use crate::resources::*;
}
