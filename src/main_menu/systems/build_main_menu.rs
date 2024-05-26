use bevy::prelude::*;
use crate::{constants, ui};
use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::ui::create;

pub fn build_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("main menu"),
        MainMenu {},
        NodeBundle {
            style: constants::styles::MAIN_MENU,
            z_index: ui::order::MAIN_MENU,
            ..default()
        },
    ))
        .with_children(|parent| {
            create::title(&asset_server, parent);
            create::button(&asset_server, parent, "Play", PlayButton {});
            create::button(&asset_server, parent, "Quit", QuitButton {});
        });
}
