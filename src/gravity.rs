use bevy::prelude::*;

use crate::movement::Acceleration;

const GRAVITY: Vec3 = Vec3::new(0.0, 0.0, -9.81);

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_gravity);
    }
}

fn update_gravity(mut query: Query<&mut Acceleration>) {
    for mut acceleration in query.iter_mut() {
        acceleration.value += GRAVITY;
    }
}
