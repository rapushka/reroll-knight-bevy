use bevy::prelude::*;

pub use table::*;
pub use field::*;
use crate::gameplay::systems::*;
use crate::infrastructure::GameplayState;
use crate::prelude::AppState;

pub mod table;
pub mod field;
pub mod config;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnCell>()

            .add_systems(OnEnter(GameplayState::Playing), (
                spawn_table,
                spawn_cells,
            )
                .chain(),
            )

            .add_systems(Update, (
                spawn_cell,
            ).run_if(on_event::<SpawnCell>()))

            .add_systems(Update, (
                snap_to_grid,
            )
                .run_if(in_state(AppState::Gameplay)),
            )
        ;
    }
}

