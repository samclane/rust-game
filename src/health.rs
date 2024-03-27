use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
