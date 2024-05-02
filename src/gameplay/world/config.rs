use std::fmt::Error;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext, ron};
use bevy::asset::io::Reader;
use bevy::prelude::*;
use std::io::Read;
use serde::Deserialize;

#[derive(Resource)]
pub struct ConfigProvider {
    pub generation: Handle<GenerationConfig>,
}

#[derive(Asset, Deserialize, TypePath)]
pub struct GenerationConfig {
    // TODO: coordintaes
    pub column_count: i32,
    pub row_count: i32,
}
