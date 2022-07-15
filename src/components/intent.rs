use super::*;

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}
