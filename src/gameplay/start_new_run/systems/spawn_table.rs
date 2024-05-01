use bevy::prelude::*;
use crate::gameplay::components::*;

pub fn spawn_table(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        InWorld {},
        spawn_mesh(&mut meshes, &mut materials)
    ));
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