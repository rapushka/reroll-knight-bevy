use bevy::prelude::*;

pub use table::*;
pub use field::*;
use crate::gameplay::systems::*;
use crate::infrastructure::GameplayState;

pub mod table;
pub mod field;
pub mod config;
pub mod view;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameplayState::Playing), (
                spawn_table,
                spawn_cells,
            )
                .chain(),
            )
        ;
    }
}

