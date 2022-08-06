use super::*;

pub fn well_fed(world: &mut World, _damage: &EffectSpawner, target: Entity) {
    if let Some(mut hc) = world.get_mut::<HungerClock>(target) {
        hc.state = HungerState::WellFed;
        hc.duration = 20;
    }
}
