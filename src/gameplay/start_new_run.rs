use bevy::prelude::*;

use crate::infrastructure::*;
use crate::ui::components::*;
use crate::ui::systems::*;
use systems::*;

pub struct StartNewRunPlugin;

mod systems;

impl Plugin for StartNewRunPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::StartNewRun), (
                show::<LoadingCurtain>,
            ).chain())

            .add_systems(PostUpdate, start_game.run_if(in_state(AppState::StartNewRun)))

        ;
    }
}