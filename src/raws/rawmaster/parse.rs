use super::*;
use crate::{SpawnParticleBurst, SpawnParticleLine};

pub fn parse_particle_line(n: &str) -> SpawnParticleLine {
    let tokens: Vec<_> = n.split(';').collect();

    SpawnParticleLine::new(
        to_cp437(tokens[0].chars().next().unwrap()),
        RGB::from_hex(tokens[1]).expect("Bad RGB"),
        tokens[2].parse::<f32>().unwrap(),
    )
}

pub fn parse_particle(n: &str) -> SpawnParticleBurst {
    let tokens: Vec<_> = n.split(';').collect();

    SpawnParticleBurst::new(
        to_cp437(tokens[0].chars().next().unwrap()),
        RGB::from_hex(tokens[1]).expect("Bad RGB"),
        tokens[2].parse::<f32>().unwrap(),
    )
}
