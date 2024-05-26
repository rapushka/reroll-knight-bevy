use bevy::prelude::*;
use crate::constants;

pub fn visualise_interaction_with_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut background_color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = constants::color::PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *background_color = constants::color::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = constants::color::DEFAULT_BUTTON.into();
            }
        };
    }
}