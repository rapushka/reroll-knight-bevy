use bevy::prelude::*;
use crate::*;
use crate::ui::systems::*;

use self::systems::*;

mod systems;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), (
                build_loading_curtain,
                build_main_menu,
                spawn_camera,
                spawn_light,
            ))

            .add_systems(PostUpdate, open_main_menu.run_if(in_state(AppState::Bootstrap)))
        ;
    }
}