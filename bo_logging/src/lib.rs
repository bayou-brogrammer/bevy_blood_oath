use bracket_terminal::prelude::*;

mod builder;
mod events;
mod logstore;

pub use crate::builder::*;
pub use crate::events::*;
pub use crate::logstore::*;
pub use crate::logstore::{clear_log, clone_log, print_log, restore_log};
pub use builder::Logger;

#[derive(Clone, Debug)]
pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}
