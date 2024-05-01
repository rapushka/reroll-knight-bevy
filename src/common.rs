use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod coordinates;

#[derive(Event)]
pub struct Clicked {
    pub entity: Entity,
}