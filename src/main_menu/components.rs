use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

impl Default for PlayButton {
    fn default() -> Self { PlayButton {} }
}

#[derive(Component)]
pub struct QuitButton {}

impl Default for QuitButton {
    fn default() -> Self { QuitButton {} }
}
