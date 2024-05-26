use bevy::prelude::*;
use crate::infrastructure::{AppState, GameplayState};
pub use systems::*;
use crate::common::coordinates::Coordinates;

mod systems;
mod components;

#[derive(Event)]
pub struct SpawnCell(pub Coordinates);