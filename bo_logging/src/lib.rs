use bracket_terminal::prelude::RGB;

mod builder;
mod events;
mod logstore;

pub use builder::Logger;

#[derive(Clone, Debug)]
pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}

pub mod prelude {
    pub use crate::builder::*;
    pub use crate::events::*;
    pub use crate::logstore::*;
    pub use crate::logstore::{clear_log, clone_log, print_log, restore_log};
}
