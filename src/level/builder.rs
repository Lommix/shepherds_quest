use bevy::prelude::*;
use bevy_rapier2d::{dynamics::RigidBody, geometry::Collider};

use crate::{
    animals::{dog::DogBundle, physics::MoveTo, sheep::SheepBundle},
    gameplay::{
        goal::{Goal, GoalBundle},
        trap::{Trap, TrapBundle},
    },
    state::{AllowedState, GameState},
};

use super::{
    loader::{LevelAsset, Tiles, TILE_SIZE},
    TileBundle,
};
pub struct LevelBuilderPlugin;
impl Plugin for LevelBuilderPlugin {
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    server: Res<AssetServer>,
) {
    let Some((entity, handle)) = query.iter().next() else {
        return;
    };

    let Some(level) = levels.get(handle) else {
        return;
    };

    if let Some(data) = &level.parsed {
        data.iter().for_each(|(pos, tile)| match tile {
            Tiles::Empty | Tiles::Sheep | Tiles::Dog => {
                let color: Color = [0.5, 0.5, 0.5, 1.].into();
                cmd.spawn(TileBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(TILE_SIZE)))),
                    material: materials.add(color.into()),
                    transform: Transform::from_translation(pos.extend(0.)),
                    ..Default::default()
                });

                if let Tiles::Dog = tile {
                    let transform = Transform::from_translation(pos.extend(0.));
                    cmd.spawn(DogBundle {
                        scene: server.load("models/pug.glb#Scene0"),
                        gltf: server.load("models/pug.glb"),
                        transform,
                        ..default()
                    });
                }

                if let Tiles::Sheep = tile {
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

        cmd.entity(entity).insert(LevelLoaded);

        // let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(2000.))));
        // let material = materials.add(Color::GREEN.into());
        // cmd.spawn(PbrBundle {
        //     mesh,
        //     material,
        //     transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        //     ..Default::default()
        // })
        // .insert(AllowedState::new(GameState::Game))
        // .insert(Name::new("ground"));

        cmd.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        });

        cmd.spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(200., -100., 100.)),
            point_light: PointLight {
                intensity: 990000.,
                radius: 8000.,
                range: 5000.,

                #[cfg(not(target_arch = "wasm32"))]
                shadows_enabled: true,

                ..Default::default()
            },
            ..default()
        });
    } else {
        next_state.set(GameState::Menu);
    }
}
