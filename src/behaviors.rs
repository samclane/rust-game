use bevy::prelude::*;

use rand_distr::{Distribution, Normal};

use rand::distributions::Uniform;
use rand::Rng;

use crate::{movement::Acceleration, schedule::InGameSet};

#[derive(Component, Debug)]
pub enum WalkerType {
    RandomWalker,
    GaussianWalker,
}

impl WalkerType {
    pub fn get_random() -> Self {
        match rand::thread_rng().gen_range(0..2) {
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
            (random_walk, gaussian_walk).in_set(InGameSet::EntityUpdates),
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

fn gaussian_walk(mut query: Query<&mut Acceleration, With<WalkerType>>) {
    let mut rng = rand::thread_rng();
    let Ok(normal) = Normal::new(0.0, 1.0) else {
        panic!("Failed to create normal distribution");
    };
    for mut acceleration in query.iter_mut() {
        let (x, z) = (normal.sample(&mut rng), normal.sample(&mut rng));
        acceleration.value += Vec3::new(x, 0., z);
    }
}
