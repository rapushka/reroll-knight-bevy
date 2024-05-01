use bevy::prelude::*;
use hud::HudPlugin;

use crate::gameplay::world::table::*;

pub mod hud;
pub mod progression;
pub mod start_new_run;
pub mod end_run;
pub mod systems;
pub mod world;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HudPlugin,
                TablePlugin,
            ))
        ;
    }
}