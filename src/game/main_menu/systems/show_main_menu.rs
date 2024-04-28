use bevy::prelude::*;
use crate::constants;
use crate::ui::components::*;
use crate::ui::create;

pub fn show_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu = commands.spawn((
        MainMenu {},
        NodeBundle { style: constants::styles::MAIN_MENU, ..default() },
    ))
        .with_children(|parent| {
            create::title(asset_server, parent);
            create::button::<PlayButton>(asset_server, parent, "Play");
            create::button::<QuitButton>(asset_server, parent, "Quit");
        }).id();

    main_menu
}
