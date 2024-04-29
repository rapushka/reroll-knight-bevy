use bevy::prelude::*;
use dependencies::*;
use infrastructure::*;
use crate::bootstrap::BootstrapPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::ui::systems::*;

mod constants;
mod ui;
mod infrastructure;
mod dependencies;
mod bootstrap;
mod main_menu;

fn main() {
    App::new()
        .init_state::<AppState>()

        .add_plugins(DependenciesPlugin)

        .add_plugins(BootstrapPlugin)
        .add_plugins(MainMenuPlugin)

        .add_systems(Update, visualise_interaction_with_buttons)

        .run();
}

