use bevy::prelude::*;
use crate::gameplay::world::config::GenerationConfigHandle;

const GENERATION_PATH: &str = "configs/config.generation.ron";

pub fn load_configs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let generation = GenerationConfigHandle(asset_server.load(GENERATION_PATH));
    commands.insert_resource(generation);
}