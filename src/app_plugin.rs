use bevy::prelude::*;

use crate::common::systems::*;
use crate::dependencies::DependenciesPlugin;
use crate::gameplay::progression::per_run::{RunProgression};
use crate::gameplay::start_new_run::StartNewRunPlugin;
use crate::prelude::*;
use crate::ui::systems::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()

            .add_event::<Clicked>()

            .add_plugins(DependenciesPlugin)

            .add_plugins((
                AssetsPlugin,
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
