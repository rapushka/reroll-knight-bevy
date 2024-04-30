use bevy::app::{App, Plugin};
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DependenciesPlugin;

impl Plugin for DependenciesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(WorldInspectorPlugin::new())
        ;
    }
}
