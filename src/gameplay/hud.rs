use bevy::prelude::*;
use components::GameplayHUD;
use systems::on_back_button_clicked;
use crate::infrastructure::AppState;
use crate::ui::components::LoadingCurtain;
use crate::ui::systems::*;

pub mod systems;
pub mod components;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), (
                show::<GameplayHUD>,
                hide::<LoadingCurtain>,
            ).chain())

            .add_systems(Update, (
                on_back_button_clicked,
            ).run_if(in_state(AppState::Gameplay)))

            .add_systems(OnExit(AppState::Gameplay), (
                show::<LoadingCurtain>,
                hide::<GameplayHUD>,
            ).chain())
        ;
    }
}


