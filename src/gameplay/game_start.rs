use bevy::prelude::*;
use bevy_aseprite::*;
use bevy_rapier2d::prelude::*;

use crate::{
    level::LevelBundle,
    state::{AllowedState, GameState},
};

use super::{
    goal::{Goal, GoalBundle},
    trap::{Trap, TrapBundle},
};

pub struct GameStartPlugin;
impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), start_level);
    }
}

fn start_level(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn(LevelBundle {
        level: server.load("levels/first.level.ron"),
        ..default()
    });
}

fn build_map(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(500.))));
    let material = materials.add(Color::GREEN.into());

    cmd.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
        ..Default::default()
    })
    .insert(AllowedState::new(GameState::Game))
    .insert(Name::new("ground"));

    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });

    cmd.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        point_light: PointLight {
            intensity: 90000.,
            radius: 8000.,
            range: 5000.,

            #[cfg(not(target_arch = "wasm32"))]
            shadows_enabled: true,

            ..Default::default()
        },
        ..default()
    });

    // wall
    let size: f32 = 200.;
    let thickness: f32 = 1.;

    cmd.spawn(GoalBundle {
        transform: Transform::from_translation(Vec3::new(0., 168., 0.)),
        goal: Goal::new(Vec2::new(size, 32.)),
        ..default()
    });

    cmd.spawn(TrapBundle {
        transform: Transform::from_translation(Vec3::new(0., -168., 0.)),
        trap: Trap::new(Vec2::new(size, 32.)),
        ..default()
    });

    [Vec3::X, -Vec3::X, Vec3::Y, -Vec3::Y]
        .iter()
        .for_each(|&dir| {
            let transform = Transform::from_translation(dir * size)
                .with_rotation(Quat::from_rotation_z(dir.angle_between(Vec3::Y)));
            let collider = Collider::cuboid(size, thickness / 2.);
            let cube = shape::Box::new(size * 2., thickness, 5.);

            cmd.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(cube)),
                material: materials.add(Color::WHITE.into()),
                transform,
                ..Default::default()
            })
            .insert(AllowedState::new(GameState::Game))
            .insert(collider)
            .insert(RigidBody::Fixed);
        });
}
