use bevy::prelude::*;
use crate::gameplay::world::config::*;

const GENERATION_PATH: &str = "configs/config.generation.ron";

pub fn load_configs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ConfigProvider {
        generation: asset_server.load(GENERATION_PATH),
    });
}