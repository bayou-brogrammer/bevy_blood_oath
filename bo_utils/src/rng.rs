use bracket_random::prelude::RandomNumberGenerator;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock() = RandomNumberGenerator::seeded(seed);
}

pub fn roll_dice(n: i32, die_type: i32) -> i32 {
    RNG.lock().roll_dice(n, die_type)
}

pub fn range(min: i32, max: i32) -> i32 {
    RNG.lock().range(min, max)
}
