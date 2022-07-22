use super::*;

pub fn particle_spawn_system(mut commands: Commands, mut particle_builder: ResMut<ParticleBuilder>) {
    for ParticleRequest { pt, color, glyph, lifetime } in particle_builder.requests.iter() {
        commands.spawn().insert_bundle(ParticleBundle::new(
            Position::new(*pt),
            Glyph::new(*glyph, *color, RenderOrder::Particle),
            ParticleLifetime::new(*lifetime, None),
        ));
    }

    particle_builder.requests.clear();
}

pub fn update_particles(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut ParticleLifetime)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_millis() as f32;
    for (entity, mut lifetime) in particles.iter_mut() {
        lifetime.lifetime_ms -= delta;
        if lifetime.lifetime_ms < 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
