use crate::prelude::*;

#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Description(pub String);

impl Description {
    pub fn new(description: &str) -> Self {
        Self(description.to_string())
    }
}
