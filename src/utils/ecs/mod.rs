use crate::prelude::*;

mod spawn_list;
pub use spawn_list::*;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

pub fn clear_entities(mut commands: Commands, q: Query<Entity>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}
