use bevy::prelude::*;

use crate::bootstrap::*;
use crate::common::Clicked;
use crate::dependencies::DependenciesPlugin;
use crate::gameplay::GameplayPlugin;
use crate::gameplay::start_new_run::StartNewRunPlugin;
use crate::infrastructure::AppState;
use crate::main_menu::*;
use crate::ui::systems::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()

            .add_event::<Clicked>()

            .add_plugins(DependenciesPlugin)

            .add_plugins(BootstrapPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(StartNewRunPlugin)
            .add_plugins(GameplayPlugin)

            .add_systems(Update, visualise_interaction_with_buttons)
            .add_systems(Update, click_on_pressed_button)
        ;
    }
}
