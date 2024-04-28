use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use crate::game::*;

mod game;
mod constants;
mod ui;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum AppState {
    #[default]
    Bootstrap,
    MainMenu,
}

fn main() {
    App::new()
        .init_state::<AppState>()

        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GamePlugin)

        .run();
}

