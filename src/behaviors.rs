use bevy::prelude::*;

use rand_distr::{Distribution, Normal, StandardNormal};

use rand::distributions::Uniform;
use rand::Rng;

use noise::{NoiseFn, Perlin};

use crate::{movement::Acceleration, schedule::InGameSet};

pub struct BehaviorsPlugin;

#[derive(Component)]
pub enum WalkType {
    Random,
    Gaussian,
    Normal,
    Perlin,
}

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_walks).in_set(InGameSet::EntityUpdates));
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

fn get_walk(walk_type: &WalkType) -> fn(&mut Acceleration, &Transform) {
    match walk_type {
        WalkType::Random => random_walk,
        WalkType::Gaussian => gaussian_walk,
        WalkType::Normal => normal_walk,
        WalkType::Perlin => perlin_walk,
    }
}

fn handle_walks(mut walker_query: Query<(&Transform, &mut Acceleration, &WalkType)>) {
    for (transform, mut acceleration, walk_type) in walker_query.iter_mut() {
        let walk = get_walk(walk_type);
        walk(&mut acceleration, transform);
    }
}

fn random_walk(acceleration: &mut Acceleration, _transform: &Transform) {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-1.0, 1.0);
    let (x, z) = (rng.sample(dist), rng.sample(dist));
    acceleration.value += Vec3::new(x, 0., z);
}

fn gaussian_walk(acceleration: &mut Acceleration, _transform: &Transform) {
    let distribution = StandardNormal;
    let mut rng = rand::thread_rng();
    let (x, z) = (distribution.sample(&mut rng), distribution.sample(&mut rng));
    acceleration.value += Vec3::new(x, 0., z);
}

fn normal_walk(acceleration: &mut Acceleration, _transform: &Transform) {
    let distribution = Normal::new(0.0, 0.5).unwrap();
    let mut rng = rand::thread_rng();
    let (x, z) = (distribution.sample(&mut rng), distribution.sample(&mut rng));
    acceleration.value += Vec3::new(x, 0., z);
}

fn perlin_walk(acceleration: &mut Acceleration, transform: &Transform) {
    let multiplier: f32 = 1.;
    let perlin = Perlin::new(1);
    let (x, z) = (
        perlin.get([transform.translation.x as f64, 0., 0.]),
        perlin.get([0., 0., transform.translation.z as f64]),
    );
    acceleration.value += Vec3::new(x as f32, 0., z as f32) * multiplier;
}
