use bevy::prelude::*;

#[derive(Component)]
pub struct Preparing {}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppState {
    #[default]
    Bootstrap,
    MainMenu,
}