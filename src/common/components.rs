use bevy::prelude::Component;
use crate::common::coordinates::*;
use crate::prelude::layer::Layer;

#[derive(Component)]
pub struct OnTableCoordinates {
    pub coordinates: Coordinates,
    pub layer: Layer,
}

impl OnTableCoordinates {
    pub fn new(coordinates: Coordinates, layer: Layer) -> Self {
        Self { coordinates, layer }
    }
}