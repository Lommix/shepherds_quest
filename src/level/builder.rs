use bevy::prelude::*;
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider};

use crate::{
    animals::{dog::DogBundle, llama::LLamaBundle, physics::MoveTo, sheep::SheepBundle},
    goal::{Goal, GoalBundle},
    level::{LevelBundle, TILE_SIZE},
    state::GameState,
    trap::{Trap, TrapBundle},
    ui::Dialog,
};

use super::{
    loader::{LevelAsset, Tiles},
    LevelLoaded, Score, TileBundle,
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

        let gras_material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(server.load("sprites/grass.png")),
            ..default()
        });

        let wall_material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(server.load("textures/cobble_1.png")),
            ..default()
        });

        let wall_mesh = meshes.add(
            Mesh::from(shape::Box::new(TILE_SIZE, TILE_SIZE, 5.))
                .with_generated_tangents()
                .unwrap(),
        );

        let flat_mesh = meshes.add(
            Mesh::from(shape::Quad::new(Vec2::splat(TILE_SIZE)))
                .with_generated_tangents()
                .unwrap(),
        );

        cmd.entity(entity).with_children(|cmd| {
            data.iter().for_each(|(pos, tile)| match tile {
                Tiles::Empty | Tiles::Sheep | Tiles::Dog | Tiles::Llama => {
                    cmd.spawn(TileBundle {
                        mesh: flat_mesh.clone(),
                        transform: Transform::from_translation(pos.extend(0.)),
                        material: gras_material.clone(),
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
                        (0..level.sheeps_per_spawn).for_each(|_i| {
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
                    cmd.spawn(TileBundle {
                        mesh: wall_mesh.clone(),
                        material: wall_material.clone(),
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
                transform: Transform::from_translation((level.size.unwrap() / 2.).extend(225.)),
                point_light: PointLight {
                    color: Color::rgb_u8(177, 230, 250),
                    intensity: 1990000.,
                    radius: 0.,
                    range: 500.,
                    #[cfg(not(target_arch = "wasm32"))]
                    shadows_enabled: true,
                    ..Default::default()
                },
                ..default()
            });

            info!("enter game");
            next_state.set(GameState::Game);
        });

        cmd.spawn(MaterialMeshBundle {
            transform: Transform::from_translation(Vec3::new(
                level.size.unwrap().x / 2.,
                level.size.unwrap().y / 2.,
                -1.,
            )),
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(9000.)))),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
            ..default()
        })
        .insert(Name::new("Ground"));

        cmd.entity(entity).insert(LevelLoaded);
        dialog.sections[0].value = level.intro.clone();
    });
}
