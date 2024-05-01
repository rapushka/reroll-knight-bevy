use bevy::prelude::*;

use crate::gameplay::hud::components::*;
use crate::gameplay::hud::systems::on_back_button_clicked;
use crate::gameplay::start_new_run::systems::*;
use crate::gameplay::systems::*;
use crate::gameplay::world::table::*;
use crate::infrastructure::AppState;
use crate::ui::components::LoadingCurtain;
use crate::ui::systems::*;

pub mod hud;
pub mod progression;
pub mod start_new_run;
pub mod end_run;
pub mod systems;
pub mod world;

pub struct GameplayPlugin;

const TARGET_STATE: AppState = AppState::Gameplay;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TablePlugin,
            ))

            .add_systems(OnEnter(TARGET_STATE), (
                show::<GameplayHUD>,
                hide::<LoadingCurtain>,
            ).chain())

            .add_systems(Update, (
                on_back_button_clicked,
            ).run_if(in_state(TARGET_STATE)))

            .add_systems(OnExit(TARGET_STATE), (
                show::<LoadingCurtain>,
                hide::<GameplayHUD>,
            ).chain())
        ;
    }
}
