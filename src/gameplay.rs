use bevy::prelude::*;
use crate::gameplay::hud::components::*;

use crate::infrastructure::AppState;
use crate::ui::components::LoadingCurtain;
use crate::ui::systems::*;

pub struct GameplayPlugin;

pub mod hud;
pub mod start_new_run;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), (
                show::<GameplayHUD>,
                hide::<LoadingCurtain>,
            ).chain())
        ;
    }
}