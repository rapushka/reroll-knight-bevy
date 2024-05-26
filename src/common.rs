use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod coordinates;
pub mod layer;
pub mod utils;

#[derive(Event)]
pub struct Clicked {
    pub entity: Entity,
}