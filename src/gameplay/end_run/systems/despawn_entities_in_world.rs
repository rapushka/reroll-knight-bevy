use bevy::prelude::*;
use crate::gameplay::components::InWorld;

pub fn despawn_entities_in_world(
    mut entities: Query<Entity, With<InWorld>>,
    mut commands: Commands,
) {
    for entity in &mut entities {
        commands.entity(entity).despawn();
    }
}
