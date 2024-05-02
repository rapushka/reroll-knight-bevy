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

pub fn test_resource_loaded(
    configs: Res<Assets<GenerationConfig>>,
    config_provider: Res<ConfigProvider>,
) {
    let handle = &config_provider.generation;

    if let Some(config) = configs.get(handle) {
        println!("field sizes: {}, {}", config.column_count, config.row_count);
    }
}