use bevy::prelude::*;
use bevy_rapier3d::dynamics::ExternalForce;
use noise::{NoiseFn, Perlin};
use rand::distributions::{Distribution, Uniform};
use rand::{distributions::Standard, Rng};
use rand_distr::Normal;

use crate::schedule::InGameSet;
use crate::spaceship::Spaceship;

const SEEK_SPEED: f32 = 0.25;

pub struct BehaviorsPlugin;

#[derive(Component)]
pub enum WalkType {
    Random,
    Gaussian,
    Normal,
    Perlin,
}

impl WalkType {
    pub fn walk(&self, ext_force: &mut ExternalForce, transform: &Transform) {
        match self {
            WalkType::Random => {
                let mut rng = rand::thread_rng();
                let dist = Uniform::new(-1.0, 1.0);
                let (x, z) = (rng.sample(dist), rng.sample(dist));
                ext_force.force += Vec3::new(x, 0., z);
            }
            WalkType::Gaussian => {
                let distribution = Standard;
                let mut rng = rand::thread_rng();
                let (x, z) = (distribution.sample(&mut rng), distribution.sample(&mut rng));
                ext_force.force += Vec3::new(x, 0., z);
            }
            WalkType::Normal => {
                let distribution = Normal::new(0.0, 0.5).unwrap();
                let mut rng = rand::thread_rng();
                let (x, z) = (distribution.sample(&mut rng), distribution.sample(&mut rng));
                ext_force.force += Vec3::new(x, 0., z);
            }
            WalkType::Perlin => {
                let mut rng = rand::thread_rng();
                let seed = rng.gen_range(0..u32::MAX);
                let perlin = Perlin::new(seed);
                let (x, z) = (
                    perlin.get([transform.translation.x as f64, 0., 0.]),
                    perlin.get([0., 0., transform.translation.z as f64]),
                );
                ext_force.force += Vec3::new(x as f32, 0., z as f32);
            }
        }
    }
}

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_walks, handle_seek_player).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn handle_walks(mut walker_query: Query<(&Transform, &mut ExternalForce, &WalkType)>) {
    for (transform, mut ext_force, walk_type) in walker_query.iter_mut() {
        walk_type.walk(&mut ext_force, transform);
    }
}

fn handle_seek_player(
    mut walker_query: Query<(&Transform, &mut ExternalForce), With<WalkType>>,
    player_query: Query<&Transform, With<Spaceship>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for (transform, mut ext_force) in walker_query.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        ext_force.force += direction.normalize() * SEEK_SPEED;
    }
}

pub fn get_random_walk_type() -> WalkType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => WalkType::Random,
        1 => WalkType::Gaussian,
        2 => WalkType::Normal,
        _ => WalkType::Perlin,
    }
}
