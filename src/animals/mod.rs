use bevy::prelude::*;

pub mod animations;
pub mod dog;
pub mod llama;
pub mod physics;
pub mod sheep;
pub mod telegraph;

pub struct SheepPlugin;
impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            dog::DogPlugin,
            sheep::SheepBehaviorPlugin,
            animations::AnimalAnimationPlugin,
            physics::AnimalPhysicsPlugin,
            llama::LlamaPlugin,
            telegraph::TelegraphPlugin,
        ));
    }
}
