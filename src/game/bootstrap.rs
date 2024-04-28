use bevy::prelude::*;
use crate::*;

use self::systems::*;

mod systems;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), (
                spawn_loading_curtain,
                spawn_camera,
                spawn_light,
            ))

            .add_systems(PostUpdate, open_main_menu.run_if(in_state(AppState::Bootstrap)))
        ;
    }
}