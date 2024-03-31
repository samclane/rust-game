use bevy::prelude::*;

use rand::Rng;

const STAR_COUNT: usize = 100;

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
    let mut rng = rand::thread_rng();
    let mesh = meshes.add(Sphere::default());
    let material = materials.add(StandardMaterial {
        emissive: Color::WHITE,
        ..Default::default()
    });
    for _ in 0..STAR_COUNT {
        let translation = Vec3::new(
            rng.gen_range(-25.0..25.0),
            rng.gen_range(-25.0..25.0),
            rng.gen_range(-25.0..25.0),
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
