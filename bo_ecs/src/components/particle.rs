use crate::prelude::*;
use bo_utils::impl_new;
use bracket_geometry::prelude::Point;

#[derive(Clone)]
pub struct ParticleAnimation {
    pub timer: f32,
    pub step_time: f32,
    pub path: Vec<Point>,
    pub current_step: usize,
}

#[derive(Component, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
    pub animation: Option<ParticleAnimation>,
}

impl_new!(ParticleLifetime, lifetime_ms: f32, animation: Option<ParticleAnimation>);
