use super::*;

pub fn particle_to_tile(world: &mut World, tile_idx: usize, effect: &EffectSpawner) {
    if let EffectType::Particle { lifespan, color, glyph } = effect.effect_type {
        world.resource_scope(|world, mut builder: Mut<ParticleBuilder>| {
            let map = world.resource::<Map>();
            builder.request(map.index_to_point2d(tile_idx), color, glyph, lifespan);
        });
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
        Targets::Single(target),
    );
}

pub fn add_hit_miss_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle {
            glyph: to_cp437('‼'),
            color: ColorPair::new(CYAN, BLACK),
            lifespan: 200.0,
        },
        Targets::Single(target),
    );
}

pub fn add_heal_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle {
            glyph: to_cp437('‼'),
            color: ColorPair::new(GREEN, BLACK),
            lifespan: 200.0,
        },
        Targets::Single(target),
    );
}
