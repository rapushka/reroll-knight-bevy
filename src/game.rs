use bevy::prelude::*;
use self::bootstrap::*;

mod bootstrap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BootstrapPlugin)
        ;
    }
}
