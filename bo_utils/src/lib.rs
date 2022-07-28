mod bterm;
mod ecs;
mod macros;
mod render;

pub use crate::render::color::*;

pub mod prelude {
    pub use crate::bterm::*;
    pub use crate::ecs::*;
    pub use crate::macros::*;
    pub use crate::render::*;
}
