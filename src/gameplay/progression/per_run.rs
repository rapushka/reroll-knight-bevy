use bevy::prelude::*;
use crate::common::coordinates::Coordinates;

#[derive(Event)]
pub struct StartRun; // TODO: is this event needed?

#[derive(Resource)]
pub struct RunProgression {
    pub table_coordinates: Coordinates,
}

impl Default for RunProgression {
    fn default() -> Self {
        Self {
            table_coordinates: Coordinates::ZERO,
        }
    }
}