use bevy::prelude::*;

use crate::common::Clicked;
use crate::gameplay::hud::components::BackButton;
use crate::infrastructure::AppState;

pub fn on_back_button_clicked(
    buttons: Query<Entity, With<BackButton>>,
    mut event_reader: EventReader<Clicked>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.entity) {
            next_state.set(AppState::MainMenu);
        }
    }
}