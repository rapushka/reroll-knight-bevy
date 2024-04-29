use bevy::prelude::{Color, Commands, default, NodeBundle, Res};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use crate::constants;
use crate::ui::components::LoadingCurtain;
use crate::ui::create;

pub fn build_loading_curtain(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("loading curtain"),
        LoadingCurtain {},
        NodeBundle {
            style: constants::styles::LOADING_CURTAIN,
            background_color: Color::BLACK.into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            create::text(&asset_server, "Loading...", parent, constants::FONT_SIZE);
        });
}
