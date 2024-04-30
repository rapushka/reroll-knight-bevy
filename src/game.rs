use bevy::prelude::*;

use crate::bootstrap::*;
use crate::common::Clicked;
use crate::dependencies::DependenciesPlugin;
use crate::infrastructure::AppState;
use crate::main_menu::*;
use crate::ui::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()

            .add_event::<Clicked>()

            .add_plugins(DependenciesPlugin)

            .add_plugins(BootstrapPlugin)
            .add_plugins(MainMenuPlugin)

            .add_systems(Update, visualise_interaction_with_buttons)
            .add_systems(Update, click_on_pressed_button)
        ;
    }
}
