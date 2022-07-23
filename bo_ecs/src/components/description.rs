use crate::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: &str) -> Self {
        Self(description.to_string())
    }
}
