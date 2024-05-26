use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_inspector_egui::quick::*;

pub struct DependenciesPlugin;

impl Plugin for DependenciesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins,
                // WorldInspectorPlugin::new(),
                EditorPlugin::default(),
            ))
        ;
    }
}