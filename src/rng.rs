use crate::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}
