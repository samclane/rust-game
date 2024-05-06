use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{asteroids::Asteroid, health::Health, schedule::InGameSet, spaceship::Spaceship};

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}

impl CollisionDamage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_events.in_set(InGameSet::CollisionDetection))
            .add_systems(
                Update,
                (display_events, apply_collision_damage)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            )
            .add_event::<CollisionEvent>();
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Collision detected: {:?}", collision_event);
    }
    for contact_force_event in contact_force_events.read() {
        println!("Contact force detected: {:?}", contact_force_event);
    }
}

pub fn apply_collision_damage(
    rapier_context: Res<RapierContext>,
    mut ship_query: Query<
        (&mut Health, &mut CollisionDamage, Entity),
        (With<Spaceship>, Without<Asteroid>),
    >,
    mut asteroid_query: Query<
        (&mut Health, &mut CollisionDamage, Entity),
        (With<Asteroid>, Without<Spaceship>),
    >,
) {
    for (mut ship_health, mut ship_collision_damage, ship) in ship_query.iter_mut() {
        for (mut asteroid_health, mut asteroid_collision_damage, asteroid) in
            asteroid_query.iter_mut()
        {
            if let Some(_contact_pair) = rapier_context.contact_pair(ship, asteroid) {
                let collision_force = 1.0;
                let collision_damage = collision_force * 0.1;
                ship_health.value -= collision_damage;
                ship_collision_damage.amount += collision_damage;
                asteroid_health.value -= collision_damage;
                asteroid_collision_damage.amount += collision_damage;
            }
        }
    }
}
