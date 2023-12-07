use self::loader::LevelAsset;
use crate::state::{AllowedState, GameState};
use bevy::prelude::*;

pub mod builder;
pub mod loader;
pub mod progress;
pub mod transistion;

pub const TILE_SIZE: f32 = 8.;

pub const CAMPAIGN_LEVELS: [&str; 1] = [
    "levels/1.level.ron",
    // "levels/2.level.ron",
    // "levels/3.level.ron",
    // "levels/4.level.ron",
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
    }
}

pub enum Current {
    Campaign(usize),
    Custom(Handle<LevelAsset>),
}

impl Default for Current {
    fn default() -> Self {
        Self::Campaign(0)
    }
}

#[derive(Resource, Default)]
pub struct Levels {
    levels: Vec<Handle<LevelAsset>>,
    current: Current,
}

impl Levels {
    pub fn new(levels: Vec<Handle<LevelAsset>>) -> Self {
        Self {
            levels,
            current: Current::default(),
        }
    }

    pub fn current(&self) -> Handle<LevelAsset> {
        match self.current {
            Current::Campaign(id) => self.levels[id].clone(),
            Current::Custom(ref handle) => handle.clone(),
        }
    }

    pub fn is_last_or_custom(&self) -> bool{
        match self.current {
            Current::Campaign(id) => id >= self.levels.len() - 1,
            Current::Custom(_) => true,
        }
    }

    pub fn next(&self) -> Option<Handle<LevelAsset>> {
        match self.current {
            Current::Campaign(id) => {
                if id < self.levels.len() - 1 {
                    Some(self.levels[id + 1].clone())
                } else {
                    None
                }
            }
            Current::Custom(_) => None,
        }
    }

    pub fn set(&mut self, handle: Handle<LevelAsset>) {
        match self.levels.iter().enumerate().find(|(_, h)| h == &&handle) {
            Some((i, _)) => {
                self.current = Current::Campaign(i);
            }
            None => {
                self.current = Current::Custom(handle);
            }
        }
    }
}

#[derive(Component)]
pub struct LevelLoaded;

// #[derive(Resource, Default)]
// pub struct CurrentLevel {
//     campaign_level: Option<usize>,
//     pub handle: Handle<LevelAsset>,
// }
//
// impl CurrentLevel {
//     pub fn is_custom(&self) -> bool {
//         self.campaign_level.is_none()
//     }
//
//     pub fn current(&self) -> Handle<LevelAsset> {
//         self.handle.clone()
//     }
//
//     pub fn set(&mut self, handle: Handle<LevelAsset>) {
//         self.handle = handle;
//     }
//
//     pub fn last_campaign_level(&self) -> bool {
//         self.campaign_level.is_some() && self.campaign_level.unwrap() >= CAMPAIGN_LEVELS.len() - 1
//     }
//
//     pub fn next_campaign_level(&mut self) -> Option<usize> {
//         match self.campaign_level {
//             Some(id) => {
//                 if id < CAMPAIGN_LEVELS.len() - 1 {
//                     self.campaign_level = Some(id + 1);
//                     Some(id + 1)
//                 } else {
//                     None
//                 }
//             }
//             None => None,
//         }
//     }
// }

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
