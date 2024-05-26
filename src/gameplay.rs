use bevy::prelude::*;
use hud::HudPlugin;

use crate::gameplay::world::*;
use crate::infrastructure::GameplayState;
use crate::prelude::AppState;

pub mod hud;
pub mod progression;
pub mod start_new_run;
pub mod end_run;
pub mod systems;
pub mod world;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameplayState>()

            .add_plugins((
                HudPlugin,
                WorldPlugin,
            ))

            .add_systems(OnExit(AppState::Gameplay), reset_gameplay_state)
        ;
    }
}

fn reset_gameplay_state(
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
) {
    next_gameplay_state.set(GameplayState::Starting);
}