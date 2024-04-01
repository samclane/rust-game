use std::ops::Range;

use bevy::prelude::*;

use rand::Rng;

const STAR_COUNT: usize = 250;
const STAR_SPAWN_RANGE_X: Range<f32> = -250.0..250.0;
const STAR_SPAWN_RANGE_Y: Range<f32> = -250.0..250.0;
const STAR_SPAWN_RANGE_Z: Range<f32> = -250.0..250.0;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_stars);
    }
}

fn spawn_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Should be a light source but I was playing with materials
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Sphere::new(0.1));
    let material = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(230000.0, 90000.0, 30000.0),
        ..Default::default()
    });
    for _ in 0..STAR_COUNT {
        let translation = Vec3::new(
            rng.gen_range(STAR_SPAWN_RANGE_X),
            rng.gen_range(STAR_SPAWN_RANGE_Y),
            rng.gen_range(STAR_SPAWN_RANGE_Z),
        );
        let transform = Transform::from_translation(translation);
        commands.spawn((PbrBundle {
            transform: transform,
            mesh: mesh.clone(),
            material: material.clone(),
            ..default()
        },));
    }
}
