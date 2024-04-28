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
        ;
    }
}
