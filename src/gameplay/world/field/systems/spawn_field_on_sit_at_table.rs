use bevy::prelude::*;

use crate::gameplay::world::components::Table;
use crate::gameplay::world::config::*;

pub fn spawn_field_on_new_table(
    new_table: Query<Entity, Added<Table>>,
    mut commands: Commands,
) {
    for table in new_table.iter() {
        commands.entity(table).with_children(|parent| {
            parent.spawn((
                Name::new("Cell"),
            ));
        });
    }
}

pub fn test_resource_loaded() {
    let sizes = generation::FIELD_SIZES;
    println!("field sizes: {}, {}", sizes.column, sizes.row);
}