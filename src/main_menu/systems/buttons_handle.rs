use bevy::app::AppExit;
use bevy::prelude::*;

use crate::common::Clicked;
use crate::infrastructure::AppState;
use crate::main_menu::*;

pub fn on_play_button_clicked(
    buttons: Query<Entity, With<PlayButton>>,
    mut event_reader: EventReader<Clicked>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.entity) {
            next_state.set(AppState::StartNewRun);
        }
    }
}

pub fn on_quit_button_clicked(
    buttons: Query<Entity, With<QuitButton>>,
    mut event_reader: EventReader<Clicked>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.entity) {
            app_exit_event.send(AppExit);
        }
    }
}
