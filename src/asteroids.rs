use std::f32::consts::PI;
use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::asset_loader::SceneAssets;
use crate::collision_detection::{Collider, CollisionDamage};
use crate::debug::DebugEntity;
use crate::health::Health;
use crate::movement::{Acceleration, Mass, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use crate::spaceship::Spaceship;

const MASS_SCALAR: f32 = 3.0;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATION_SPEED: f32 = 2.5;
const HEALTH: f32 = 80.0;
const COLLISION_DAMAGE: f32 = 35.0;
const SCALE: Vec3 = Vec3::splat(1.);
const MAX_NUM_ASTEROIDS: usize = 1_000;
const SPAWN_RANGE: Range<f32> = 100.0..500.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    pub timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            ((spawn_asteroids, rotate_asteroids).in_set(InGameSet::EntityUpdates),),
        );
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    astroid_query: Query<&Transform, With<Asteroid>>,
    player_query: Query<&Transform, With<Spaceship>>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    let num_asteroids = astroid_query.iter().len();
    if num_asteroids >= MAX_NUM_ASTEROIDS {
        return;
    }
    let astroid_spawn_count = (MAX_NUM_ASTEROIDS - num_asteroids).min(MAX_NUM_ASTEROIDS);
    info!("Spawning {} asteroids", astroid_spawn_count);
    let player_pos = player_query.single().translation;

    for _ in 0..astroid_spawn_count {
        let (x, z) = get_random_position_around(player_pos, SPAWN_RANGE);
        let translation = Vec3::new(x, 0.0, z);

        let velocity = get_random_unit_vector() * VELOCITY_SCALAR;
        let acceleration = get_random_unit_vector() * ACCELERATION_SCALAR;

        commands.spawn((
            MovingObjectBundle {
                mass: Mass::new(MASS_SCALAR),
                velocity: Velocity::new(velocity),
                acceleration: Acceleration::new(acceleration),
                collider: Collider::new(1.0),
                model: SceneBundle {
                    scene: scene_assets.asteroids.clone(),
                    transform: Transform::from_translation(translation).with_scale(SCALE),
                    ..default()
                },
            },
            Asteroid,
            Health::new(HEALTH),
            CollisionDamage::new(COLLISION_DAMAGE),
            DebugEntity,
        ));
    }
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(ROTATION_SPEED * time.delta_seconds()));
    }
}

pub fn get_random_position_around(pos: Vec3, range: Range<f32>) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(range);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

fn get_random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero()
}
