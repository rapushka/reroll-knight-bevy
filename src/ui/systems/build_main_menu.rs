use bevy::prelude::{Commands, default, NodeBundle, Res};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use crate::constants;
use crate::ui::components::{MainMenu, PlayButton, QuitButton};
use crate::ui::create;

pub fn build_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        MainMenu {},
        NodeBundle { style: constants::styles::MAIN_MENU, ..default() },
    ))
        .with_children(|parent| {
            create::title(&asset_server, parent);
            create::button::<PlayButton>(&asset_server, parent, "Play");
            create::button::<QuitButton>(&asset_server, parent, "Quit");
        });
}
