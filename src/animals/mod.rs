use std::time::Duration;

use bevy::{gltf::Gltf, prelude::*};
use bevy_aseprite::{slice::AsepriteSlice, Aseprite};
use bevy_rapier2d::{
    control::KinematicCharacterController,
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, ColliderMassProperties, Sensor},
};
use bevy_tweening::{lens::*, *};

use crate::state::GameState;

use self::{
    physics::MoveTo,
    sheep::{SheepBundle, SheepTag},
};

pub mod dog;
pub mod physics;
pub mod sheep;

const SPAWN_COUNT: usize = 500;

pub struct SheepPlugin;
impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(physics::AnimalPhysicsPlugin);
        app.add_plugins(dog::DogPlugin);
        app.add_plugins(sheep::SheepBehaviorPlugin);
        app.add_systems(OnEnter(GameState::Game), start_level);
        app.add_systems(Update, (bounce));
        app.insert_resource(AnimalBehavior {
            alignment: 0.8,
            cohesion: 0.8,
            separation: 0.8,
            speed: 10.0,
            vision: 15.0,
            fear: 1.0,
            motivation: 0.5,
        });
    }
}

#[derive(Resource)]
pub struct AnimalBehavior {
    pub alignment: f32,
    pub cohesion: f32,
    pub separation: f32,
    pub speed: f32,
    pub vision: f32,
    pub fear: f32,
    pub motivation: f32,
}

fn start_level(mut cmd: Commands, server: Res<AssetServer>) {
    let position = crate::util::quad_formation(SPAWN_COUNT, 5.);
    (0..SPAWN_COUNT).zip(position).for_each(|(i, pos)| {
        let transform = Transform::from_translation(pos);
        cmd.spawn(SheepBundle {
            scene: server.load("models/sheep.glb#Scene0"),
            transform,
            ..default()
        })
        .insert(MoveTo::new(Vec2::ZERO));
    });
}

fn bounce(
    mut cmd: Commands,
    query: Query<
        (&Children),
        (
            With<SheepTag>,
            Added<Children>,
            Without<Animator<Transform>>,
        ),
    >,
) {
    query.iter().for_each(|(children)| {
        let random = rand::random::<u64>() % 50;
        let tween = Tween::new(
            EaseFunction::QuadraticOut,
            Duration::from_millis(300 + random),
            TransformPositionLens {
                start: Vec3::new(0., 0., 0.),
                end: Vec3::new(0., 0., 2.),
            },
        )
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
        .with_repeat_count(RepeatCount::Infinite);

        cmd.entity(children[0]).insert(Animator::new(tween));
    });
}

fn load_animation(
    mut query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
    server: Res<AssetServer>,
    gltf: Res<Assets<Gltf>>,
) {
    query.iter_mut().for_each(|mut player| {
        let animation = server.load("models/models.glb#Animation4");
        let random = (rand::random::<f32>() * 0.2) + 0.9;
        player.play(animation.clone()).repeat().set_speed(random);
    });
}
