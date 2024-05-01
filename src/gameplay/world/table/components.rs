use bevy::prelude::Component;
use crate::common::coordinates::Coordinates;

#[derive(Component)]
pub struct Table;

#[derive(Component)]
pub struct InRoomCoordinates {
    pub coordinates: Coordinates,
}