use bevy::prelude::{NextState, Query, ResMut};
use crate::infrastructure::AppState;
use crate::infrastructure::*;

pub fn open_main_menu(
    mut next_app_state: ResMut<NextState<AppState>>,
    preparing_entities: Query<&Preparing>,
) {
    if preparing_entities.is_empty() {
        next_app_state.set(AppState::MainMenu);
    }
}