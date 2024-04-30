use bevy::prelude::{NextState, Query, ResMut};
use crate::infrastructure::{AppState, Preparing};

pub fn start_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    preparing_entities: Query<&Preparing>,
) {
    if preparing_entities.is_empty() {
        next_app_state.set(AppState::Gameplay);
    }
}
