use bevy::prelude::*;

use infrastructure::*;

use crate::bootstrap::BootstrapPlugin;
use crate::game::GamePlugin;
use crate::ui::systems::*;

mod constants;
mod ui;
mod infrastructure;
mod dependencies;
mod bootstrap;
mod main_menu;
mod game;
mod common;

fn main() { App::new().add_plugins(GamePlugin).run(); }

