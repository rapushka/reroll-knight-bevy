use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use crate::game::*;
use crate::infrastructure::PreparingState;

mod game;
mod constants;
mod ui;
mod infrastructure;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum AppState {
    #[default]
    Bootstrap,
    MainMenu,
}

fn main() {
    App::new()
        .init_resource::<PreparingState<AppState>>()

        .init_state::<AppState>()

        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GamePlugin)

        .run();
}

