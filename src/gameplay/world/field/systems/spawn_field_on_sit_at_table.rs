use bevy::prelude::*;

use crate::gameplay::world::components::Table;
use crate::gameplay::world::config::*;

pub fn spawn_field_on_new_table(
    new_table: Query<Entity, Added<Table>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // let model_handle = asset_server.load("models/cell.gltf#Scene0");
    // let mesh_handle = meshes.add(model_handle);

    for table in new_table.iter() {
        commands.entity(table).with_children(|parent| {
            parent.spawn((
                Name::new("Cell"),
                SceneBundle {
                    scene: asset_server.load("models/cell.gltf#Scene0"),
                    ..default()
                },
            ));
        });
    }
}