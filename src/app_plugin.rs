use bevy::prelude::*;

use crate::bootstrap::*;
use crate::common::*;
use crate::common::systems::*;
use crate::dependencies::DependenciesPlugin;
use crate::gameplay::GameplayPlugin;
use crate::gameplay::progression::per_run::{RunProgression, StartRun};
use crate::gameplay::start_new_run::StartNewRunPlugin;
use crate::infrastructure::AppState;
use crate::main_menu::*;
use crate::ui::systems::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()

            .init_resource::<RunProgression>()

            .add_event::<Clicked>()
            .add_event::<StartRun>()

            .add_plugins(DependenciesPlugin)

            .add_plugins((
                BootstrapPlugin,
                MainMenuPlugin,
                StartNewRunPlugin,
                GameplayPlugin,
            ))

            .add_systems(Update, (
                visualise_interaction_with_buttons,
                click_on_pressed_button,
                despawn_not_in_state,
            ))
        ;
    }
}
