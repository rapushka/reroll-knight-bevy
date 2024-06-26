use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::core::Name;
use crate::constants;

pub fn title(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn((
        Name::new("title"),
        NodeBundle { style: constants::styles::TITLE, ..default() },
    ))
        .with_children(|parent| {
            text(asset_server, "the Bad Luck", parent, 64.0);
        });
}

pub fn button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: &str,
    component: C,
)
    where C: Component {
    parent.spawn((
        Name::new(format!("button [{}]", string)),
        component,
        ButtonBundle {
            style: constants::styles::BUTTON,
            background_color: constants::color::DEFAULT_BUTTON.into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            text(asset_server, string, parent, 32.0);
        });
}

pub fn text(
    asset_server: &Res<AssetServer>,
    text: &str,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent.spawn((
        Name::new(format!("text: {text}")),
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new(
                        text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size,
                            color: Color::WHITE,
                        },
                    )],
                justify: JustifyText::Center,
                ..default()
            },
            ..default()
        },
    ));
}
