use bevy::prelude::*;

use crate::{collision_detection::Collider, schedule::InGameSet};

#[derive(Component, Debug)]
pub struct Mass {
    pub value: f32,
}

impl Mass {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub mass: Mass,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct StaticObjectBundle {
    pub mass: Mass,
    pub model: SceneBundle,
    pub collider: Collider,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_positions)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn update_velocity(mut query: Query<(&mut Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (mut acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
        acceleration.value = Vec3::ZERO;
    }
}

fn update_positions(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut position) in query.iter_mut() {
        position.translation += velocity.value * time.delta_seconds();
    }
}
