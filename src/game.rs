use bevy::prelude::*;
use self::bootstrap::*;
use self::main_menu::*;

mod bootstrap;
mod main_menu;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BootstrapPlugin)
            .add_plugins(MainMenuPlugin)
        ;
    }
}
