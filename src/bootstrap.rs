use bevy::prelude::*;
use crate::*;
use crate::gameplay::hud::components::GameplayHUD;
use crate::main_menu::MainMenu;
use crate::main_menu::systems::*;
use crate::ui::systems::{build_loading_curtain, hide};

use self::gameplay::hud::systems::*;
use self::systems::*;

mod systems;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), (
                build_loading_curtain,
                build_main_menu,
                hide::<MainMenu>,
                build_gameplay_hud,
                hide::<GameplayHUD>,
                spawn_camera,
                spawn_light,
                load_configs,
            ).chain())

            .add_systems(PostUpdate, open_main_menu.run_if(in_state(AppState::Bootstrap)))
        ;
    }
}