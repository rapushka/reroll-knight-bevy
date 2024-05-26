use bevy::prelude::*;

use crate::gameplay::systems::*;
use crate::prelude::GameplayState;

pub mod components;

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameplayState::Playing), (
                spawn_table,
            ))
        ;
    }
}