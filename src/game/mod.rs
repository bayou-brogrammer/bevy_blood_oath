use crate::prelude::*;

pub mod player;

pub fn build_ai_scheduler() -> Schedule {
    Schedule::builder().build()
}
