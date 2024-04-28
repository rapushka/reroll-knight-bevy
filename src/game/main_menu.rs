use bevy::prelude::*;
use crate::AppState;
use crate::ui::components::LoadingCurtain;
use self::systems::*;

mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), (
                hide::<LoadingCurtain>, // it actually works!!!
                show_main_menu,
            ))
            .add_systems(OnExit(AppState::MainMenu), hide_main_menu)
        ;
    }
}