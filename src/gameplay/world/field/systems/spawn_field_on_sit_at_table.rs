use bevy::prelude::*;
use crate::common::components::OnTableCoordinates;
use crate::common::coordinates::Coordinates;

use crate::gameplay::world::components::Table;
use crate::gameplay::world::field::components::*;

use crate::prelude::*;

const CELLS_ORIGIN: Vec3 = Vec3::new(0.0, 0.07, 0.0); // TODO: move me to config

pub fn spawn_cells(
    mut commands: Commands,
    assets: Res<EnvironmentAssets>,
    tables: Query<Entity, With<Table>>,
) {
    let table = tables.single();
    let coordinates = Coordinates::new(0, 0);

    commands.spawn(Cell)
        .insert(Name::new("Cell"))
        .insert(SceneBundle {
            scene: assets.cell.clone(),
            ..default()
        })
        .insert(OnTableCoordinates(coordinates))
        .insert(Transform {
            translation: CELLS_ORIGIN,
            scale: Vec3::ONE * 0.1,
            ..default()
        })
        .set_parent(table);
}