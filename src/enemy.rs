use bevy::prelude::*;
use std::ops::Range;

use crate::{
    asset_loader::SceneAssets,
    asteroids::get_random_position_around,
    behaviors::get_random_walk_type,
    collision_detection::{Collider, CollisionDamage},
    debug::DebugEntity,
    health::Health,
    movement::{Acceleration, Mass, MovingObjectBundle, Velocity},
    schedule::InGameSet,
    spaceship::Spaceship,
};

#[derive(Component)]
pub struct Enemy;

#[derive(Resource, Debug)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

pub struct EnemyPlugin;

const NUM_ENEMIES: usize = 10;
const ENEMY_MASS: f32 = 1.0;
// const ENEMY_SPAWN_RANGE: Range<f32> = 100.0..500.0;
const ENEMY_SPAWN_RANGE: Range<f32> = 10.0..50.0;
const ENEMY_HEALTH: f32 = 80.0;
const ENEMY_COLLISION_DAMAGE: f32 = 35.0;
const SPAWN_TIME_SECONDS: f32 = 0.5;
const ENEMY_SCALE: Vec3 = Vec3::splat(1.);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            ((spawn_enemies, rotate_to_face_player).in_set(InGameSet::EntityUpdates),),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Spaceship>>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    let num_enemies = enemy_query.iter().len();
    if num_enemies >= NUM_ENEMIES {
        return;
    }
    let enemy_spawn_count = (NUM_ENEMIES - num_enemies).min(NUM_ENEMIES);
    info!("Spawning {} enemies", enemy_spawn_count);
    let player_pos = player_query.single().translation;
    (0..enemy_spawn_count).for_each(|_| {
        let (x, z) = get_random_position_around(player_pos, ENEMY_SPAWN_RANGE);
        let translation = Vec3::new(x, 0.0, z);
        commands.spawn((
            MovingObjectBundle {
                mass: Mass::new(ENEMY_MASS),
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(1.0),
                model: SceneBundle {
                    scene: scene_assets.aliens.clone(),
                    transform: Transform::from_translation(translation).with_scale(ENEMY_SCALE),
                    ..default()
                },
            },
            Enemy,
            Health::new(ENEMY_HEALTH),
            CollisionDamage::new(ENEMY_COLLISION_DAMAGE),
            get_random_walk_type(),
            DebugEntity,
        ));
    });
}

fn rotate_to_face_player(
    mut query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Spaceship>, Without<Enemy>)>,
) {
    if player_query.iter().count() == 0 {
        return;
    }
    let player_pos = player_query.single().translation;
    for mut enemy_transform in query.iter_mut() {
        let direction = player_pos - enemy_transform.translation;
        let rotation = Quat::from_rotation_y(-direction.x.atan2(direction.z));
        enemy_transform.rotation = rotation;
    }
}
