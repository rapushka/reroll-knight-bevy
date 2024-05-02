use bevy::prelude::*;
use crate::infrastructure::AppState;
use systems::*;

mod systems;
mod components;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), (
                test_resource_loaded
            ))

            .add_systems(Update, (
                spawn_field_on_new_table,
            ).run_if(in_state(AppState::Gameplay)))
        ;
    }
}

