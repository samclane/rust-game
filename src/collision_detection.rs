use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{health::Health, schedule::InGameSet};

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

fn apply_collision_damage(
    rapier_context: Res<RapierContext>,
    mut query: Query<(&mut Health, Entity, &CollisionDamage)>,
) {
    for (mut health, entity, collision_damage) in query.iter_mut() {
        for contact_pair_group in rapier_context.contact_pairs_with(entity) {
            if contact_pair_group.has_any_active_contacts() {
                health.value -= collision_damage.amount;
            }
        }
    }
}
