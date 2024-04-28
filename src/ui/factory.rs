use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::core::Name;

pub fn create_text(asset_server: &Res<AssetServer>, text: &str, parent: &mut ChildBuilder, font_size: f32) {
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
