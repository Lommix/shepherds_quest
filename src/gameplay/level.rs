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
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();
        app.add_systems(Update, load_level);
    }
}

#[derive(Resource, Default)]
pub struct CurrentLevel(Option<Entity>);

#[derive(Component)]
pub struct LevelLoaded;

fn load_level(
    mut cmd: Commands,
    query: Query<(Entity, &Handle<LevelAsset>), Without<LevelLoaded>>,
    levels: Res<Assets<LevelAsset>>,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some((entity, handle)) = query.iter().next() else {
        return;
    };

    let Some(level) = levels.get(handle) else {
        return;
    };
}
