use bevy::prelude::*;

use rand::distributions::Uniform;
use rand::Rng;

use crate::{movement::Acceleration, schedule::InGameSet};

#[derive(Component)]
pub struct RandomWalker;

pub struct BehaviorsPlugin;

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (random_walk).in_set(InGameSet::EntityUpdates));
    }
}

fn random_walk(mut query: Query<&mut Acceleration, With<RandomWalker>>) {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(-1.0, 1.0);
    for mut acceleration in query.iter_mut() {
        let (x, z) = (rng.sample(dist), rng.sample(dist));
        acceleration.value += Vec3::new(x, 0., z);
    }
}
