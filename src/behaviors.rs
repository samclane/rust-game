use bevy::prelude::*;

use rand_distr::{Distribution, Normal, StandardNormal};

use rand::distributions::Uniform;
use rand::Rng;

use crate::{movement::Acceleration, schedule::InGameSet};

#[derive(Component, Debug)]
pub enum WalkerType {
    RandomWalker,
    GaussianWalker,
    NormalWalker,
}

impl WalkerType {
    pub fn get_random() -> Self {
        let count = 3;
        match rand::thread_rng().gen_range(0..count) {
            1 => Self::NormalWalker,
            0 => Self::GaussianWalker,
            _ => Self::RandomWalker,
        }
    }

    pub fn new_random() -> Self {
        Self::RandomWalker
    }

    pub fn new_gaussian() -> Self {
        Self::GaussianWalker
    }
}

pub struct BehaviorsPlugin;

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (random_walk, gaussian_walk, normal_walk).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn random_walk(mut query: Query<&mut Acceleration, With<WalkerType>>) {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-1.0, 1.0);
    for mut acceleration in query.iter_mut() {
        let (x, z) = (rng.sample(dist), rng.sample(dist));
        acceleration.value += Vec3::new(x, 0., z);
    }
}

fn walk<T: Distribution<f32>>(
    mut query: Query<&mut Acceleration, With<WalkerType>>,
    distribution: T,
) {
    let mut rng = rand::thread_rng();
    for mut acceleration in query.iter_mut() {
        let (x, z) = (distribution.sample(&mut rng), distribution.sample(&mut rng));
        acceleration.value += Vec3::new(x, 0., z);
    }
}

fn gaussian_walk(query: Query<&mut Acceleration, With<WalkerType>>) {
    let distribution = StandardNormal;
    walk(query, distribution);
}

fn normal_walk(query: Query<&mut Acceleration, With<WalkerType>>) {
    let distribution = Normal::new(0.0, 0.5).unwrap();
    walk(query, distribution);
}
