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

pub mod animations;
pub mod dog;
pub mod llama;
pub mod physics;
pub mod sheep;

pub struct SheepPlugin;
impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dog::DogPlugin,
            sheep::SheepBehaviorPlugin,
            animations::AnimalAnimationPlugin,
            physics::AnimalPhysicsPlugin,
            llama::LlamaPlugin,
        ));
        app.insert_resource(AnimalBehavior {
            alignment: 1.0,
            cohesion: 1.0,
            separation: 0.5,
            speed: 23.0,
            vision: 20.0,
            fear: 1.0,
            motivation: 0.1,
            dog_speed: 50.0,
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
    pub dog_speed: f32,
}
