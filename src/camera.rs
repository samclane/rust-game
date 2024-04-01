use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::spaceship::Spaceship;

const CAMERA_DISTANCE: f32 = 120.0;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, pan_camera_to_spaceship);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        MainCamera,
        BloomSettings::NATURAL,
    ));
}

fn pan_camera_to_spaceship(
    query: Query<&Transform, With<Spaceship>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Spaceship>)>,
) {
    let Ok(spaceship_transform) = query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    camera_transform.translation =
        spaceship_transform.translation + Vec3::new(0.0, CAMERA_DISTANCE, 0.0);
}
