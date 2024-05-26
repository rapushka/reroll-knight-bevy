use bevy::prelude::*;

use crate::common::components::OnTableCoordinates;
use crate::common::coordinates::Coordinates;
use crate::gameplay::world::components::Table;
use crate::gameplay::world::field::components::*;
use crate::gameplay::world::SpawnCell;
use crate::prelude::*;

pub fn spawn_cells(
    mut event: EventWriter<SpawnCell>,
) {
    event.send(SpawnCell(Coordinates::ZERO));
}

pub fn spawn_cell(
    mut event: EventReader<SpawnCell>,
    mut commands: Commands,
    assets: Res<EnvironmentAssets>,
    tables: Query<Entity, With<Table>>,
) {
    let table = tables.single();

    for e in event.read() {
        let coordinates = e.0;

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
}