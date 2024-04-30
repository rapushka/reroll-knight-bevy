use bevy::prelude::*;
use crate::common::Clicked;

pub fn click_on_pressed_button(
    mut buttons: Query<(&Interaction, Entity), Changed<Interaction>>,
    mut clicked_event: EventWriter<Clicked>,
) {
    for (interaction, entity) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                clicked_event.send(Clicked { entity: entity });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        };
    }
}
