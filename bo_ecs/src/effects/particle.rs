use super::*;
use bo_map::prelude::Map;
use bracket_algorithm_traits::prelude::Algorithm2D;

pub struct ParticleRequest {
    pub pt: Point,
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub lifetime: f32,
}

impl_new!(ParticleRequest, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32);

pub struct ParticleBuilder {
    pub requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder { requests: Vec::new() }
    }

    pub fn request(&mut self, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32) {
        self.requests.push(ParticleRequest::new(pt, color, glyph, lifetime));
    }
}

pub fn particle_to_tile(
    map: Res<Map>,
    mut builder: ResMut<ParticleBuilder>,
    mut particle_events: ResMut<Events<ParticleEvent>>,
) {
    for ParticleEvent { tile_idx, effect } in particle_events.drain() {
        if let EffectType::Particle { lifespan, color, glyph } = effect.effect_type {
            builder.request(map.index_to_point2d(tile_idx), color, glyph, lifespan);
        }
    }
}

pub fn add_damage_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle { glyph: to_cp437('‼'), color: ColorPair::new(ORANGE, BLACK), lifespan: 200.0 },
        Targets::Single { target },
    );
}
