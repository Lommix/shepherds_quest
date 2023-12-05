use self::loader::LevelAsset;
use crate::{
    state::{AllowedState, GameState},
};
use bevy::{
    prelude::*,
};


pub mod builder;
pub mod loader;
pub mod progress;
pub mod transistion;

pub const CAMPAIGN_LEVELS: [&str; 4] = [
    "levels/first.level.ron",
    "levels/second.level.ron",
    "levels/third.level.ron",
    "levels/fourth.level.ron",
];

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            loader::LevelLoaderPlugin,
            builder::LevelBuilderPlugin,
            transistion::LevelTransitionPlugin,
            progress::LevelProgressPlugin,
        ));
        app.init_resource::<Score>();
        app.init_resource::<CurrentLevel>();
    }
}

#[derive(Component)]
pub struct LevelLoaded;

#[derive(Resource, Default)]
pub struct CurrentLevel(pub usize);

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

#[derive(Resource, Default)]
pub struct Score {
    pub lost: usize,
    pub saved: usize,
}

impl Score {
    pub fn reset(&mut self) {
        self.lost = 0;
        self.saved = 0;
    }

    fn reset_score(mut score: ResMut<Score>) {
        score.lost = 0;
        score.saved = 0;
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
