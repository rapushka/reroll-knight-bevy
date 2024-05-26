use bevy::prelude::Component;
use crate::common::coordinates::*;

#[derive(Component)]
pub struct OnTableCoordinates(pub Coordinates);