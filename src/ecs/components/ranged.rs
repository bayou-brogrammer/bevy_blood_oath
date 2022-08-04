use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}
