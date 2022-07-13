#[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod multi_thread;

#[cfg(target_arch = "wasm32")]
pub use single_thread::*;

#[cfg(not(target_arch = "wasm32"))]
pub use multi_thread::*;

use super::*;
use specs::prelude::World;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: &mut World);
    fn setup(&mut self, ecs: &mut World);
}

pub fn new_ticking_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
    construct_dispatcher!((FovSystem, "fov", &[]));
}
