use bevy::prelude::*;

use crate::gameplay::world::components::Table;
use crate::gameplay::world::field::components::Cell;
use crate::prelude::EnvironmentAssets;

pub fn spawn_field_on_new_table(
    new_table: Query<Entity, Added<Table>>,
    mut commands: Commands,
    assets: Res<EnvironmentAssets>,
) {
    // let model_handle = asset_server.load("models/cell.gltf#Scene0");
    // let mesh_handle = meshes.add(model_handle);

    for table in new_table.iter() {
        // TODO: can't you just set parent??
        commands.entity(table).with_children(|parent| {
            parent.spawn((
                Name::new("Cell"),
                Cell,
                SceneBundle {
                    scene: assets.cell.clone(),
                    ..default()
                },
            ));
        });
    }
}