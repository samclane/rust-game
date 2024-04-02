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
            (draw_colliders, draw_kinematics)
                .chain()
                .after(InGameSet::EntityUpdates),
        )
        .add_systems(Update, update_config);
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

fn update_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    // Toggle gizmos
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        config.enabled = !config.enabled;
    }
    // Control line width with arrow keys
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        config.line_width += 5. * time.delta_seconds();
        config.line_width = config.line_width.clamp(0., 50.);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        config.line_width -= 5. * time.delta_seconds();
        config.line_width = config.line_width.clamp(0., 50.);
    }
}
