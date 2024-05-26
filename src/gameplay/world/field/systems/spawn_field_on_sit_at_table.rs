use bevy::prelude::*;

use crate::gameplay::world::components::Table;
use crate::gameplay::world::field::components::Cell;
use crate::prelude::*;

pub fn spawn_field_on_new_table(
    new_table: Query<Entity, Added<Table>>,
    mut commands: Commands,
    assets: Res<EnvironmentAssets>,
) {
    for table in new_table.iter() {
        commands.spawn((
            Name::new("Cell"),
            Cell,
            SceneBundle {
                scene: assets.cell.clone(),
                ..default()
            },
        )).set_parent(table);
    }
}