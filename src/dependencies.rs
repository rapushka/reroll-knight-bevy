use bevy::app::{App, Plugin};
use bevy::DefaultPlugins;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::gameplay::world::config::GenerationConfig;

pub struct DependenciesPlugin;

impl Plugin for DependenciesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins,
                new_ron_deserializer(),
                WorldInspectorPlugin::new(),
            ))
        ;
    }
}

fn new_ron_deserializer() -> RonAssetPlugin<GenerationConfig> {
    RonAssetPlugin::<GenerationConfig>::new(&["generation.ron"])
}