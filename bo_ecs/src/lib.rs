mod bundle;
mod components;
mod effects;
mod events;
mod setup;
mod state;

pub mod queries;

pub mod prelude {
    pub use bevy_ecs::prelude::{Bundle, Component, Entity};

    pub use crate::bundle::*;
    pub use crate::components::*;
    pub use crate::effects::*;
    pub use crate::events::*;
    pub use crate::queries::*;
    pub use crate::setup::*;
    pub use crate::state::*;
}
