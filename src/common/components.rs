use bevy::prelude::Component;
use crate::common::coordinates::Coordinates as CoordinatesValue;

#[derive(Component)]
pub struct Coordinates {
    pub coordinates: CoordinatesValue,
}