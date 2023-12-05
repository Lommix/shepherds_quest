use bevy::prelude::*;
use bevy_nine_slice_ui::NineSliceUiTexture;

use crate::{
    animals::sheep::SheepTag,
    level::{loader::LevelAsset, LevelLoaded},
    menu::LevelSelectorButton,
    state::{AllowedState, GameState},
};

pub mod goal;
pub mod trap;
pub mod ui;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
    }
}
