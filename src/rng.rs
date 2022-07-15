use crate::prelude::*;
use parking_lot::RwLock;

lazy_static! {
    pub static ref RNG: RwLock<RandomNumberGenerator> = RwLock::new(RandomNumberGenerator::new());
}
