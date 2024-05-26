use bevy::math::Vec3;
use crate::prelude::coordinates::Coordinates;

pub const FONT_SIZE: f32 = 32.0;

pub mod color {
    use bevy::prelude::*;

    pub const DEFAULT_BUTTON: Color = Color::hsl(0.0, 0.0, 0.15);
    pub const HOVERED_BUTTON: Color = Color::hsl(0.0, 0.0, 0.25);
    pub const PRESSED_BUTTON: Color = Color::hsl(0.0, 0.0, 0.10);
}

pub mod styles {
    use bevy::prelude::*;

    pub const LOADING_CURTAIN: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.padding = UiRect::all(Val::Px(10.0));
        style.flex_direction = FlexDirection::ColumnReverse;
        style
    };

    pub const MAIN_MENU: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.row_gap = Val::Px(8.0);
        style
    };

    pub const GAMEPLAY_HUD: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.padding = UiRect::all(Val::Px(10.0));
        style
    };

    pub const BUTTON: Style = {
        let mut style = Style::DEFAULT;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(200.0);
        style.height = Val::Px(80.0);
        style
    };

    pub const TITLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_direction = FlexDirection::Row;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(300.0);
        style.height = Val::Px(300.0);
        style
    };
}

pub const CELLS_ORIGIN: Vec3 = Vec3::new(0.0, 0.07, 0.0);
pub const CELLS_SCALE: f32 = 0.17;

pub const FIELD_SIZES: Coordinates = Coordinates::new(3, 6); 