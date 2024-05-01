use bevy::prelude::*;

use crate::gameplay::world::table::components::InRoomCoordinates;
use crate::gameplay::world::table::SitAtTable;
use crate::infrastructure::*;

pub fn spawn_table(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_reader: EventReader<SitAtTable>,
) {
    for sit_at_table in event_reader.read() {
        let coordinates = sit_at_table.coordinates;

        commands.spawn((
            Name::new(format!("table: {}", coordinates)),
            OnAppState(AppState::Gameplay),
            InRoomCoordinates { coordinates: coordinates },
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