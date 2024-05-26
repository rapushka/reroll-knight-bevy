use bevy::prelude::*;

use crate::gameplay::progression::per_run::RunProgression;
use crate::gameplay::world::components::{InRoomCoordinates, Table};
use crate::infrastructure::{AppState, OnAppState};
use crate::prelude::EnvironmentAssets;

pub fn spawn_table(
    run_progression: Res<RunProgression>,
    mut commands: Commands,
    assets: Res<EnvironmentAssets>,
) {
    let coordinates = run_progression.current_table;

    commands.spawn(Table)
        .insert(Name::new(format!("table: {}", coordinates)))
        .insert(InRoomCoordinates { coordinates })
        .insert(SceneBundle {
            scene: assets.table.clone(),
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(0.0, -0.07, 0.0),
            ..default()
        })
        .insert(OnAppState(AppState::Gameplay))
    ;
}