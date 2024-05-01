use bevy::prelude::*;
use crate::common::components::Coordinates;
use crate::gameplay::progression::per_run::*;
use crate::infrastructure::*;

pub fn spawn_table(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_reader: EventReader<StartRun>,
    run_progression: Res<RunProgression>,
) {
    for event in event_reader.read() {
        commands.spawn((
            OnAppState(AppState::Gameplay),
            Coordinates { coordinates: run_progression.table_coordinates },
            spawn_mesh(&mut meshes, &mut materials),
        ));
    }
}

fn spawn_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> MaterialMeshBundle<StandardMaterial> {
    PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::DARK_GREEN),
        ..default()
    }
}