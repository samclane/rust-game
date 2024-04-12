use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    debug::DebugEntity,
    health::Health,
    movement::{Acceleration, Mass, StaticObjectBundle},
    schedule::InGameSet,
    state::GameState,
};

const NUM_PLANETS: usize = 20;
const PLANET_COLLISION_DAMAGE: f32 = 10.0;
const PLANET_RANGE_MASS: Range<f32> = 5_000.0..20_000.0;
const SPAWN_RANGE_X: Range<f32> = -500.0..500.0;
const SPAWN_RANGE_Z: Range<f32> = -500.0..500.0;
const PLANET_RANGE_SCALE: Range<f32> = 15.5..25.5;
const HEALTH: f32 = 10_000_000.0;
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
    let mut planets: Vec<Transform> = vec![];
    let mut attempts = 0;
    let max_attempts = NUM_PLANETS * 10; // Prevent infinite loop

    while planets.len() < NUM_PLANETS && attempts < max_attempts {
        attempts += 1;
        let translation = Vec3::new(
            rng.gen_range(SPAWN_RANGE_X),
            0.0,
            rng.gen_range(SPAWN_RANGE_Z),
        );
        let scale = rng.gen_range(PLANET_RANGE_SCALE);
        let transform = Transform::from_translation(translation).with_scale(Vec3::splat(scale));

        if planets.iter().any(|planet_transform| {
            let distance = (planet_transform.translation - transform.translation).length();
            distance < (2. * transform.scale.x) + (2. * planet_transform.scale.x)
        }) {
            continue;
        }

        let mass = Mass::new(rng.gen_range(PLANET_RANGE_MASS));
        let collider = Collider::new(transform.scale.x * 2.);
        let model = SceneBundle {
            scene: scene_assets.planets.clone(),
            transform,
            ..default()
        };

        commands.spawn((
            StaticObjectBundle {
                mass,
                model,
                collider,
            },
            Planet,
            CollisionDamage::new(PLANET_COLLISION_DAMAGE),
            Health::new(HEALTH),
            DebugEntity,
        ));

        planets.push(transform);
    }
}

fn attract_objects_to_planets(
    query: Query<(&Transform, &Mass), With<Planet>>,
    mut moving_query: Query<(&mut Acceleration, &Transform, &Mass), Without<Planet>>,
) {
    for (planet_transform, planet_mass) in query.iter() {
        for (mut acceleration, transform, mass) in moving_query.iter_mut() {
            let direction = planet_transform.translation - transform.translation;
            let distance: f32 = direction.length();
            let force = (planet_mass.value * mass.value) / (distance.powi(2) + f32::EPSILON);
            acceleration.value += direction.normalize() * force;
        }
    }
}

fn rotate_planets(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(ROTATION_SPEED * time.delta_seconds()));
    }
}
