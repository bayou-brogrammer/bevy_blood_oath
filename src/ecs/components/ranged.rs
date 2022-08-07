use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Ranged(pub i32);

#[derive(Component, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}

impl_new!(AreaOfEffect, radius: i32);
