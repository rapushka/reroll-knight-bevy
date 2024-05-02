use bevy::prelude::*;
use crate::common::coordinates::Coordinates;
use crate::gameplay::start_new_run::systems::spawn_table;
use crate::gameplay::systems::sit_at_table_from_progress;
use crate::infrastructure::AppState;

pub mod components;

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SitAtTable>()

            .add_systems(OnEnter(AppState::Gameplay), (
                sit_at_table_from_progress,
            ))

            .add_systems(Update, (
                spawn_table,
            ).run_if(on_event::<SitAtTable>()))
        ;
    }
}

#[derive(Event)]
pub struct SitAtTable {
    /// Coordinates of the Table in the Room
    pub coordinates: Coordinates,
}