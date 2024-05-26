use bevy::prelude::*;

use crate::infrastructure::*;
use crate::ui::components::*;
use crate::ui::systems::*;
use self::systems::*;

pub struct StartNewRunPlugin;

pub mod systems;

impl Plugin for StartNewRunPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::StartNewRun), (
                show::<LoadingCurtain>,
                create_new_progression,
                start_run,
            ).chain())
        ;
    }
}