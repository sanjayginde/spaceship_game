use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
