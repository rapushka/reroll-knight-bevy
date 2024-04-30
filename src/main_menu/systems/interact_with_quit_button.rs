use bevy::prelude::*;
use bevy::app::AppExit;
use crate::common::Clicked;
use crate::main_menu::QuitButton;

pub fn on_quit_button_clicked(
    mut buttons: Query<Entity, With<QuitButton>>,
    mut event_reader: EventReader<Clicked>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.entity) {
            app_exit_event.send(AppExit);
        }
    }
}