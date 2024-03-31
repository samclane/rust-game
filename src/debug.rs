use bevy::prelude::*;

use crate::{
    collision_detection::Collider,
    movement::{Acceleration, Velocity},
    schedule::InGameSet,
};

#[derive(Component, Debug)]
pub struct DebugEntity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (print_positions, draw_colliders, draw_kinematics)
                .chain()
                .after(InGameSet::EntityUpdates),
        );
    }
}

fn print_positions(query: Query<(Entity, &Transform), With<DebugEntity>>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at {:?}", entity, transform.translation);
    }
}

fn draw_colliders(mut gizmos: Gizmos, query: Query<(&Collider, &Transform), With<DebugEntity>>) {
    for (collider, transform) in query.iter() {
        gizmos.sphere(
            transform.translation,
            Quat::IDENTITY,
            collider.radius,
            Color::WHITE,
        );
    }
}

fn draw_kinematics(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Velocity, &Acceleration), With<DebugEntity>>,
) {
    for (transform, velocity, acceleration) in query.iter() {
        gizmos.arrow(
            transform.translation,
            transform.translation + velocity.value,
            Color::WHITE,
        );
        gizmos.arrow(
            transform.translation,
            transform.translation + acceleration.value,
            Color::RED,
        );
    }
}
