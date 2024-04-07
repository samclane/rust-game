use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::spaceship::Spaceship;

const CAMERA_DISTANCE_INIT: f32 = 120.0;
const CAMERA_LERP_SPEED: f32 = 2.;
const CAMERA_SCROLL_FACTOR: f32 = 70.0;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (pan_camera_to_spaceship, zoom_camera_controls).chain(),
        );
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
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE_INIT, 0.0)
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
    time: Res<Time>,
) {
    let Ok(spaceship_transform) = query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    camera_transform.translation = camera_transform.translation.lerp(
        spaceship_transform.translation + Vec3::new(0.0, camera_transform.translation.y, 0.0),
        CAMERA_LERP_SPEED * time.delta_seconds(),
    );

    camera_transform.look_at(spaceship_transform.translation, Vec3::Z);
}

fn zoom_camera_controls(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    for event in scroll_evr.read() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                camera_transform.translation + Vec3::new(0.0, event.y * CAMERA_SCROLL_FACTOR, 0.0),
                CAMERA_LERP_SPEED * time.delta_seconds(),
            );
        }
    }
}
