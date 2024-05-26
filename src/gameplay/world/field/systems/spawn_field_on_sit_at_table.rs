use bevy::prelude::*;
use crate::common::components::OnTableCoordinates;
use crate::common::coordinates::Coordinates;

use crate::gameplay::world::components::Table;
use crate::gameplay::world::field::components::*;
use crate::prelude::*;

pub fn spawn_field_on_new_table(
    new_table: Query<Entity, Added<Table>>,
    mut commands: Commands,
    assets: Res<EnvironmentAssets>,
) {
    for table in new_table.iter() {
        let coordinates = Coordinates::new(0, 0);

        commands.spawn(Cell)
            .insert(Name::new("Cell"))
            .insert(SceneBundle {
                scene: assets.cell.clone(),
                ..default()
            })
            .insert(OnTableCoordinates(coordinates))
            .insert(Transform {
                scale: Vec3::ONE * 0.1,
                ..default()
            })
            .set_parent(table);
    }
}