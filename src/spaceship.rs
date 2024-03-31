use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::asset_loader::SceneAssets;
use crate::collision_detection::{Collider, CollisionDamage};
use crate::debug::DebugEntity;
use crate::health::Health;
use crate::movement::{Acceleration, Mass, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use crate::state::GameState;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 0.25;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_HEALTH: f32 = 100.0;
const SPACESHIP_COLLISION_DAMAGE: f32 = 100.0;
const SPACESHIP_RADIUS: f32 = 1.0;
const SPACESHIP_MASS: f32 = 1.0;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.;
const MISSILE_HEALTH: f32 = 1.0;
const MISSILE_COLLISION_DAMAGE: f32 = 5.0;
const MISSILE_FIRE_RATE: f32 = 0.5;
const MISSILE_MASS: f32 = 1.0;
const MISSILE_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceShipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct SpaceshipMissileFireRate {
    pub timer: Timer,
}

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(OnEnter(GameState::GameOver), spawn_spaceship)
            .add_systems(
                Update,
                (
                    spaceship_movement_controls,
                    spaceship_weapon_controls,
                    spaceship_shield_controls,
                )
                    .chain()
                    .in_set(InGameSet::UserInput),
            )
            .add_systems(Update, spaceship_destroyed.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            mass: Mass::new(SPACESHIP_MASS),
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE),
        SpaceshipMissileFireRate {
            timer: Timer::from_seconds(MISSILE_FIRE_RATE, TimerMode::Once),
        },
        DebugEntity,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);

    transform.rotate_local_z(roll);

    // Update the spaceship's velocity based on the new direction
    velocity.value += -transform.forward() * movement; // Negate the forward vector to move in the direction the spaceship is facing
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut SpaceshipMissileFireRate), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
) {
    let Ok((transform, mut fire_rate)) = query.get_single_mut() else {
        return;
    };
    fire_rate.timer.tick(time.delta());
    if keyboard_input.pressed(KeyCode::Space) && fire_rate.timer.finished() {
        commands.spawn((
            MovingObjectBundle {
                mass: Mass::new(MISSILE_MASS),
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
            Health::new(MISSILE_HEALTH),
            CollisionDamage::new(MISSILE_COLLISION_DAMAGE),
        ));
        fire_rate.timer.reset();
    }
}

fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(spaceship) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(SpaceShipShield);
    }
}

fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>,
) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
