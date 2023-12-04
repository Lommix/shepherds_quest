use crate::{
    level::loader::LevelAsset,
    ron_asset_loader,
    state::{AllowedState, GameState},
};
use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;

const TILE_SIZE: f32 = 32.;

pub struct LevelPlugin;
