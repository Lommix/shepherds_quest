use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;

use crate::state::{AllowedState, GameState};

use self::loader::LevelAsset;
pub mod loader;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::LevelLoaderPlugin);
    }
}

#[derive(Bundle)]
pub struct LevelBundle {
    pub level: Handle<LevelAsset>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub allowed_game_states: AllowedState,
    pub name: Name,
}

impl Default for LevelBundle {
    fn default() -> Self {
        Self {
            level: Handle::default(),
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::HIDDEN,
            view_visibility: ViewVisibility::HIDDEN,
            transform: Transform::IDENTITY,
            global_transform: GlobalTransform::IDENTITY,
            allowed_game_states: AllowedState::new(GameState::Game),
            name: Name::new("level"),
        }
    }
}
