use bevy::prelude::*;

use infrastructure::*;

use crate::bootstrap::BootstrapPlugin;
use crate::app_plugin::AppPlugin;
use crate::ui::systems::*;

mod constants;
mod ui;
mod infrastructure;
mod dependencies;
mod bootstrap;
mod main_menu;
mod gameplay;
mod app_plugin;
mod common;

fn main() { App::new().add_plugins(AppPlugin).run(); }

