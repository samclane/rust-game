use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::schedule::InGameSet;

#[derive(Component, Debug)]
pub struct DebugEntity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (draw_kinematics).chain().after(InGameSet::EntityUpdates),
        )
        .add_systems(Update, update_config);
    }
}

fn draw_kinematics(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Velocity, &ExternalForce), With<DebugEntity>>,
) {
    for (transform, velocity, ext_force) in query.iter() {
        let acceleration: Vec3 = ext_force.force / 1.; // mass_prop.mass;
        gizmos.arrow(
            transform.translation,
            transform.translation + velocity.linvel,
            Color::WHITE,
        );
        gizmos.arrow(
            transform.translation,
            transform.translation + acceleration,
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
