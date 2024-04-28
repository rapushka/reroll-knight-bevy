pub const FONT_SIZE: f32 = 32.0;

pub mod styles {
    use bevy::prelude::*;

    pub const MAIN_MENU: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style
    };
}