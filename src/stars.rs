use std::ops::Range;

use bevy::prelude::*;

use rand::Rng;

const STAR_COUNT: usize = 2500;
const STAR_SPAWN_RANGE_X: Range<f32> = -500.0..500.0;
const STAR_SPAWN_RANGE_Y: Range<f32> = -250.0..250.0;
const STAR_SPAWN_RANGE_Z: Range<f32> = -500.0..500.0;
const STAR_SIZE_RANGE: Range<f32> = 0.1..0.5;
const BASE_COLOR: Color = Color::rgb_linear(230000.0, 90000.0, 30000.0);
const POINT_LIGHT_INTENSITY: f32 = 4000.0;
const POINT_LIGHT_RADIUS: f32 = 1000.0;
const POINT_LIGHT_COLOR: Color = Color::ANTIQUE_WHITE;

#[derive(Component, Debug)]
pub struct Star;

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
    for _ in 0..STAR_COUNT {
        let translation = Vec3::new(
            rng.gen_range(STAR_SPAWN_RANGE_X),
            rng.gen_range(STAR_SPAWN_RANGE_Y),
            rng.gen_range(STAR_SPAWN_RANGE_Z),
        );
        let transform = Transform::from_translation(translation);
        let size = rng.gen_range(STAR_SIZE_RANGE);
        let material = materials.add(StandardMaterial {
            emissive: BASE_COLOR * size, // bigger stars are brighter
            ..Default::default()
        });
        commands
            .spawn((
                PbrBundle {
                    transform,
                    mesh: meshes.add(Sphere::new(size)),
                    material: material.clone(),
                    ..default()
                },
                Star,
            ))
            .with_children(|children| {
                children.spawn(PointLightBundle {
                    point_light: PointLight {
                        intensity: POINT_LIGHT_INTENSITY,
                        radius: POINT_LIGHT_RADIUS,
                        color: POINT_LIGHT_COLOR,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}
