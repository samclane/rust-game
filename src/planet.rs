use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{Acceleration, Mass, StaticObjectBundle},
    schedule::InGameSet,
    state::GameState,
};

const NUM_PLANETS: usize = 3;
const PLANET_COLLISION_DAMAGE: f32 = 10.0;
const PLANET_RANGE_MASS: Range<f32> = 1.0..10.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const PLANET_RANGE_SCALE: Range<f32> = 0.5..1.5;
const HEALTH: f32 = 1000.0;
const ROTATION_SPEED: f32 = 1.5;

#[derive(Component, Debug)]
pub struct Planet;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_planets)
            .add_systems(OnEnter(GameState::GameOver), spawn_planets)
            .add_systems(
                Update,
                (attract_objects_to_planets, rotate_planets).in_set(InGameSet::EntityUpdates),
            );
    }
}

fn spawn_planets(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_PLANETS {
        let translation = Vec3::new(
            rng.gen_range(SPAWN_RANGE_X),
            0.0,
            rng.gen_range(SPAWN_RANGE_Z),
        );
        let transform = Transform::from_translation(translation)
            .with_scale(Vec3::splat(rng.gen_range(PLANET_RANGE_SCALE)));
        commands.spawn((
            StaticObjectBundle {
                mass: Mass::new(rng.gen_range(PLANET_RANGE_MASS)),
                model: SceneBundle {
                    scene: scene_assets.planets.clone(),
                    transform: transform,
                    ..Default::default()
                },
                collider: Collider::new(transform.scale.x * 0.5),
            },
            Planet,
            CollisionDamage::new(PLANET_COLLISION_DAMAGE),
            Health::new(HEALTH),
        ));
    }
}

fn attract_objects_to_planets(
    query: Query<(&Transform, &Mass), With<Planet>>,
    mut moving_query: Query<(&mut Acceleration, &Transform, &Mass), Without<Planet>>,
) {
    for (planet_transform, planet_mass) in query.iter() {
        for (mut acceleration, transform, mass) in moving_query.iter_mut() {
            let direction = planet_transform.translation - transform.translation;
            let distance = direction.length();
            let force = (planet_mass.value * mass.value) / distance.powi(2);
            acceleration.value += direction.normalize() * force;
        }
    }
}

fn rotate_planets(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(ROTATION_SPEED * time.delta_seconds()));
    }
}
