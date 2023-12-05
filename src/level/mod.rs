use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;

use crate::state::{AllowedState, GameState};

use self::loader::LevelAsset;
mod builder;
pub mod loader;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>();
        app.add_plugins(loader::LevelLoaderPlugin);
        app.add_plugins(builder::LevelBuilderPlugin);
        app.init_resource::<CurrentLevel>();
        app.add_systems(Update, start_level.run_if(in_state(GameState::Menu)));
    }
}


#[derive(Component)]
pub struct LevelLoaded;

#[derive(Resource, Default)]
pub struct CurrentLevel(Option<Handle<LevelAsset>>);
impl CurrentLevel {
    pub fn set(&mut self, level: Handle<LevelAsset>) {
        self.0 = Some(level);
    }
    pub fn level(&self) -> Option<Handle<LevelAsset>> {
        self.0.clone()
    }
}

#[derive(Event)]
pub struct LoadLevelEvent(Handle<LevelAsset>);
impl LoadLevelEvent {
    pub fn new(level: Handle<LevelAsset>) -> Self {
        Self(level)
    }
}

fn start_level(
    mut cmd: Commands,
    mut events: EventReader<LoadLevelEvent>,
    mut state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let Some(event) = events.read().next() else {
        return;
    };

    current_level.set(event.0.clone());
    info!("loading level {:?}", &event.0);

    cmd.spawn(LevelBundle {
        level: event.0.clone(),
        ..default()
    });

    state.set(GameState::Game);
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

#[derive(Bundle)]
pub struct TileBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub allowed_game_states: AllowedState,
    pub name: Name,
}

impl Default for TileBundle {
    fn default() -> Self {
        Self {
            mesh: Handle::default(),
            material: Handle::default(),
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::HIDDEN,
            view_visibility: ViewVisibility::HIDDEN,
            transform: Transform::IDENTITY,
            global_transform: GlobalTransform::IDENTITY,
            allowed_game_states: AllowedState::new(GameState::Game),
            name: Name::new("tile"),
        }
    }
}
