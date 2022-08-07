use crate::prelude::*;

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

pub struct ParticleRequest {
    pub pt: Point,
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub lifetime: f32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SpawnParticleLine {
    pub glyph: FontCharType,
    pub color: RGB,
    pub lifetime_ms: f32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SpawnParticleBurst {
    pub glyph: FontCharType,
    pub color: RGB,
    pub lifetime_ms: f32,
}

impl_new!(SpawnParticleLine, glyph: FontCharType, color: RGB, lifetime_ms: f32);
impl_new!(SpawnParticleBurst, glyph: FontCharType, color: RGB, lifetime_ms: f32);
impl_new!(ParticleRequest, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32);

#[derive(Default)]
pub struct ParticleBuilder {
    pub requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder { ParticleBuilder { requests: Vec::new() } }

    pub fn request(&mut self, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32) {
        self.requests.push(ParticleRequest::new(pt, color, glyph, lifetime));
    }
}
