use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct EnvironmentAssets {
    #[asset(path = "models/table.gltf#Scene0")]
    pub table: Handle<Scene>,

    #[asset(path = "models/cell.gltf#Scene0")]
    pub cell: Handle<Scene>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::MainMenu)
                    .load_collection::<EnvironmentAssets>()
            )
        ;
    }
}
