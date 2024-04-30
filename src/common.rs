use bevy::prelude::*;

#[derive(Event)]
pub struct Clicked {
    pub entity: Entity,
}