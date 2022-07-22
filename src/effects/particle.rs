use super::*;

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
        EffectType::Particle {
            glyph: to_cp437('‼'),
            color: ColorPair::new(ORANGE, BLACK),
            lifespan: 200.0,
        },
        Targets::Single { target },
    );
}
