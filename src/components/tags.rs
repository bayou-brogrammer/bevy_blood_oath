use super::*;

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Player {
    pub id: Entity,
}

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Door;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Colonist;
