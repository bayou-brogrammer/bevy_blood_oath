use crate::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum EquipmentSlot {
    Melee,
    Shield,
}

#[derive(Component, Clone, Debug)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Component, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

impl_new!(Equippable, slot: EquipmentSlot);
impl_new!(Equipped, owner: Entity, slot: EquipmentSlot);
