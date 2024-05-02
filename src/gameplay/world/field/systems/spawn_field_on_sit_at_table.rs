use bevy::prelude::*;
use crate::gameplay::world::components::Table;
use crate::gameplay::world::config::{GenerationConfig, GenerationConfigHandle};
use crate::gameplay::world::SitAtTable;

pub fn spawn_field_on_new_table(
    new_table: Query<Entity, Added<Table>>,
    mut commands: Commands,
    generation_config: Res<GenerationConfigHandle>,
    mut configs: ResMut<Assets<GenerationConfig>>,
) {
    if let Some(gen_config) = configs.remove(generation_config.0.id()) {
        println!("field sizes: {}, {}", gen_config.column_count, gen_config.row_count);
    }

    for table in new_table.iter() {
        commands.entity(table).with_children(|parent| {
            parent.spawn((
                Name::new("Cell"),
            ));
        });
    }
}