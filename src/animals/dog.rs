use bevy::{gltf::Gltf, input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_aseprite::{slice::AsepriteSlice, Aseprite};
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, ColliderMassProperties},
};

use crate::state::GameState;

use super::{physics::MoveTo, AnimalBehavior};

pub struct DogPlugin;
impl Plugin for DogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_debug_dog);
        app.add_systems(Update, move_dogs);
    }
}

fn spawn_debug_dog(mut cmd: Commands, server: Res<AssetServer>) {
    let transform = Transform::from_translation(Vec3::new(0., -100., 0.));
    cmd.spawn(DogBundle {
        scene: server.load("models/pug.glb#Scene0"),
        transform,
        ..default()
    });
}

fn move_dogs(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut Velocity, &MoveTo, &Transform), With<DogTag>>,
    animal_behavior: Res<AnimalBehavior>,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut velocity, move_to, transform)| {
            let direction = move_to.postion() - transform.translation.truncate();
            let distance = direction.length();

            if distance < 10.0 {
                cmd.entity(entity).remove::<MoveTo>();
                velocity.linvel = Vec2::ZERO;
                return;
            }

            velocity.linvel = direction.normalize_or_zero() * animal_behavior.speed * 3.;
        });
}

#[derive(Component)]
pub struct DogTag;

#[derive(Bundle)]
pub struct DogBundle {
    pub body: RigidBody,
    pub collider: Collider,
    pub scene: Handle<Scene>,
    pub dog_tag: DogTag,
    pub velocity: Velocity,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    pub name: Name,
}

impl Default for DogBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::Dynamic,
            collider: Collider::ball(2.),
            velocity: Velocity::default(),
            scene: Handle::default(),
            dog_tag: DogTag,
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::HIDDEN,
            view_visibility: ViewVisibility::HIDDEN,
            transform: Transform::IDENTITY,
            global_transform: GlobalTransform::IDENTITY,
            name: Name::new("dog"),
        }
    }
}
