use bevy::prelude::*;

use crate::AppState;
use crate::ui::components::{LoadingCurtain, MainMenu};
use crate::ui::systems::show_hide::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), (
                show::<MainMenu>,
                hide::<LoadingCurtain>,
            ).chain())

            .add_systems(OnExit(AppState::MainMenu), hide::<MainMenu>)
        ;
    }
}