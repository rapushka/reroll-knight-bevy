use bevy::prelude::*;
use infrastructure::*;

use crate::app_plugin::AppPlugin;

pub mod prelude {
    pub use super::{
        constants::*,
        ui::*,
        infrastructure::*,
        dependencies::*,
        bootstrap::*,
        main_menu::*,
        gameplay::*,
        app_plugin::*,
        common::*,
        assets::*,
    };
}

mod constants;
mod ui;
mod infrastructure;
mod dependencies;
mod bootstrap;
mod main_menu;
mod gameplay;
mod app_plugin;
mod common;
mod assets;

fn main() { App::new().add_plugins(AppPlugin).run(); }