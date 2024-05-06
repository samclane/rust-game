use std::ops::Range;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets, collision_detection::CollisionDamage, debug::DebugEntity,
    health::Health, state::GameState,
};

pub const G: f32 = 6.67430e-11;
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
            .add_systems(Update, attract_objects);
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

        let mass = ColliderMassProperties::Density(rng.gen_range(PLANET_RANGE_MASS));
        let collider = Collider::ball(2.0); // Don't know why this is 2.0
        let model = SceneBundle {
            scene: scene_assets.planets.clone(),
            transform,
            ..default()
        };

        commands.spawn((
            mass,
            model,
            collider,
            Velocity {
                angvel: Vec3::splat(ROTATION_SPEED),
                linvel: Vec3::ZERO,
            },
            Planet,
            CollisionDamage::new(PLANET_COLLISION_DAMAGE),
            Health::new(HEALTH),
            DebugEntity,
            RigidBody::Dynamic,
        ));

        planets.push(transform);
    }
}

fn attract_objects(
    mut planet_query: Query<(&Transform, &Collider), With<Planet>>,
    mut rigid_body_query: Query<(&Transform, &Collider, &mut ExternalForce), Without<Planet>>,
) {
    for (planet_transform, planet_collider) in planet_query.iter_mut() {
        for (transform, entity_collider, mut ext_force) in rigid_body_query.iter_mut() {
            let distance = (planet_transform.translation - transform.translation).length();
            let mut force = (planet_transform.translation - transform.translation).normalize()
                * (planet_collider.raw.mass_properties(10000000.0).mass()
                    * entity_collider.raw.mass_properties(1.0).mass()
                    / distance.powi(2))
                * G;
            force.y = 0.0;
            ext_force.force += force;
        }
    }
}
