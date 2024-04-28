use bevy::prelude::*;
use crate::constants;
use crate::ui::components::*;
use crate::ui::create;

pub fn spawn_loading_curtain(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    build_loading_curtain(&mut commands, &asset_server);
}

pub fn build_loading_curtain(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
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
            create::text(asset_server, "Loading...", parent, constants::FONT_SIZE);
        })
        .id()
}
