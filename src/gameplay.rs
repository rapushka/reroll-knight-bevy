use bevy::prelude::*;

use crate::gameplay::hud::components::*;
use crate::gameplay::hud::systems::on_back_button_clicked;
use crate::gameplay::end_run::systems::*;
use crate::infrastructure::AppState;
use crate::ui::components::LoadingCurtain;
use crate::ui::systems::*;

pub mod hud;
pub mod start_new_run;
pub mod end_run;
mod components;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
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
                despawn_entities_in_world,
                hide::<GameplayHUD>,
            ).chain())
        ;
    }
}
