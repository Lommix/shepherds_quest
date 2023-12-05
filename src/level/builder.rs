use bevy::prelude::*;
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider};

use crate::{
    animals::{dog::DogBundle, llama::LLamaBundle, physics::MoveTo, sheep::SheepBundle},
    goal::{Goal, GoalBundle},
    level::LevelBundle,
    menu::LevelSelectorButton,
    state::{AllowedState, GameState},
    trap::{Trap, TrapBundle},
    ui::Dialog,
};

use super::{
    loader::{LevelAsset, Tiles, TILE_SIZE},
    CurrentLevel, LevelLoaded, Score, TileBundle, CAMPAIGN_LEVELS,
};

pub struct LevelBuilderPlugin;
impl Plugin for LevelBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_level);
        app.add_event::<LoadLevelEvent>();
        app.add_systems(Update, start_level);
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
    mut score: ResMut<Score>,
) {
    let Some(event) = events.read().next() else {
        return;
    };

    info!("loading level {:?}", &event.0);

    score.reset();
    cmd.spawn(LevelBundle {
        level: event.0.clone(),
        ..default()
    });

    state.set(GameState::Game);
}

fn load_level(
    mut cmd: Commands,
    query: Query<(Entity, &Handle<LevelAsset>), Without<LevelLoaded>>,
    levels: Res<Assets<LevelAsset>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut dialog: Query<&mut Text, With<Dialog>>,
    server: Res<AssetServer>,
) {
    query.iter().for_each(|(entity, handle)| {
        let Some(level) = levels.get(handle) else {
            return;
        };

        let Some(data) = &level.parsed else {
            warn!("Failed to load from file {:?}", handle);
            next_state.set(GameState::Menu);
            return;
        };

        let Ok(mut dialog) = dialog.get_single_mut() else {
            return;
        };

        cmd.entity(entity).with_children(|cmd| {
            data.iter().for_each(|(pos, tile)| match tile {
                Tiles::Empty | Tiles::Sheep | Tiles::Dog | Tiles::Llama => {
                    let color: Color = [0.0, 1.0, 0.2, 1.].into();
                    cmd.spawn(TileBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(TILE_SIZE)))),
                        material: materials.add(color.into()),
                        transform: Transform::from_translation(pos.extend(0.)),
                        ..Default::default()
                    });

                    if matches!(tile, Tiles::Dog) {
                        let transform = Transform::from_translation(pos.extend(0.));
                        cmd.spawn(DogBundle {
                            scene: server.load("models/pug.glb#Scene0"),
                            gltf: server.load("models/pug.glb"),
                            transform,
                            ..default()
                        });
                    }

                    if matches!(tile, Tiles::Llama) {
                        let transform = Transform::from_translation(pos.extend(0.));
                        cmd.spawn(LLamaBundle {
                            scene: server.load("models/llama.glb#Scene0"),
                            gltf: server.load("models/llama.glb"),
                            transform,
                            ..default()
                        });
                    }

                    if matches!(tile, Tiles::Sheep) {
                        (0..level.sheeps_per_spawn).for_each(|i| {
                            let transform = Transform::from_translation(pos.extend(0.));
                            cmd.spawn(SheepBundle {
                                scene: server.load("models/sheep.glb#Scene0"),
                                gltf: server.load("models/sheep.glb"),
                                transform,
                                ..default()
                            })
                            .insert(MoveTo::new(*pos));
                        });
                    }
                }
                Tiles::Wall => {
                    let color: Color = [0.5, 0.5, 0.5, 1.].into();
                    cmd.spawn(TileBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(TILE_SIZE, TILE_SIZE, 5.))),
                        material: materials.add(color.into()),
                        transform: Transform::from_translation(pos.extend(0.)),
                        ..Default::default()
                    })
                    .insert(Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.))
                    .insert(RigidBody::Fixed);
                }
                Tiles::Trap => {
                    cmd.spawn(TrapBundle {
                        transform: Transform::from_translation(pos.extend(0.)),
                        trap: Trap::new(Vec2::splat(TILE_SIZE)),
                        ..Default::default()
                    });
                }
                Tiles::Goal => {
                    cmd.spawn(GoalBundle {
                        transform: Transform::from_translation(pos.extend(0.)),
                        goal: Goal::new(Vec2::splat(TILE_SIZE)),
                        ..Default::default()
                    });
                }
            });
            cmd.spawn(PointLightBundle {
                transform: Transform::from_translation((level.size.unwrap() / 2.).extend(180.)),
                point_light: PointLight {
                    intensity: 990000.,
                    radius: 99000.,
                    range: 5000.,
                    #[cfg(not(target_arch = "wasm32"))]
                    shadows_enabled: true,
                    ..Default::default()
                },
                ..default()
            });

            info!("enter game");
            next_state.set(GameState::Game);
        });

        cmd.entity(entity).insert(LevelLoaded);
        dialog.sections[0].value = level.intro.clone();
    });
}
