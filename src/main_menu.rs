use bevy::prelude::*;

use crate::infrastructure::AppState;
use crate::ui::components::{LoadingCurtain, MainMenu};
use crate::ui::systems::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app // some changes
            .add_systems(OnEnter(AppState::MainMenu), (
                show::<MainMenu>,
                hide::<LoadingCurtain>,
            ).chain())

            .add_systems(OnExit(AppState::MainMenu), hide::<MainMenu>)
        ;
    }
}