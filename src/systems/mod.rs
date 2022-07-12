use crate::prelude::*;

pub mod dispatcher;
pub use dispatcher::UnifiedDispatcher;

pub fn build() -> Box<dyn dispatcher::UnifiedDispatcher + 'static> {
    dispatcher::new()
}
