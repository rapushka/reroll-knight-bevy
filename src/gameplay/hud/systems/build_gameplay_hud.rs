use bevy::prelude::*;

use crate::{constants, ui::{self, create}};
use crate::gameplay::hud::components::*;

pub fn build_gameplay_hud(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("gameplay hud"),
        GameplayHUD {},
        NodeBundle {
            style: constants::styles::GAMEPLAY_HUD,
            z_index: ui::order::GAMEPLAY_HUD,
            ..default()
        },
    ))
        .with_children(|parent| {
            create::button(&asset_server, parent, "<", BackButton {});
        });
}