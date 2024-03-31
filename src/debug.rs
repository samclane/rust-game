use bevy::prelude::*;

use crate::schedule::InGameSet;

#[derive(Component, Debug)]
pub struct DebugEntity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_positions.after(InGameSet::EntityUpdates));
    }
}

fn print_positions(query: Query<(Entity, &Transform), With<DebugEntity>>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at {:?}", entity, transform.translation);
    }
}
