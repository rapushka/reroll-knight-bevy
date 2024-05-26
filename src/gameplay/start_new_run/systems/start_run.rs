use bevy::prelude::*;

use crate::infrastructure::{AppState, Preparing};
use crate::prelude::GameplayState;

pub fn start_run(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
    preparing_entities: Query<&Preparing>,
) {
    if preparing_entities.is_empty() {
        next_app_state.set(AppState::Gameplay);
        next_gameplay_state.set(GameplayState::Playing);
    }
}