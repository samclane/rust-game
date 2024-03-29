use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::StaticObjectBundle,
    state::GameState,
};

const NUM_PLANETS: usize = 3;
const PLANET_RADIUS: f32 = 1.0;
const PLANET_COLLISION_DAMAGE: f32 = 10.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const HEALTH: f32 = 1000.0;

#[derive(Component, Debug)]
pub struct Planet;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_planets)
            .add_systems(OnEnter(GameState::GameOver), spawn_planets);
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
        commands.spawn((
            StaticObjectBundle {
                model: SceneBundle {
                    scene: scene_assets.planets.clone(),
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                },
                collider: Collider::new(PLANET_RADIUS),
            },
            Planet,
            CollisionDamage::new(PLANET_COLLISION_DAMAGE),
            Health::new(HEALTH),
        ));
    }
}
