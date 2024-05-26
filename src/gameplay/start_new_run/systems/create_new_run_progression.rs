use bevy::prelude::*;

use crate::gameplay::progression::per_run::RunProgression;

pub fn create_new_progression(
    mut commands: Commands,
) {
    commands.insert_resource(RunProgression::default());
}