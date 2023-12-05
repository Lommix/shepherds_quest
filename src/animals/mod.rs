use bevy::prelude::*;

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
            sheep_speed: 32.0,
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
    pub sheep_speed: f32,
    pub vision: f32,
    pub fear: f32,
    pub motivation: f32,
    pub dog_speed: f32,
}
