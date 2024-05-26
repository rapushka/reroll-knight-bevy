use bevy::prelude::*;
use crate::common::coordinates::Coordinates;

#[derive(Resource)]
pub struct RunProgression {
    pub current_table: Coordinates,
}

impl Default for RunProgression {
    fn default() -> Self {
        Self {
            current_table: Coordinates::ZERO,
        }
    }
}