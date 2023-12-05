use bevy::{gltf::Gltf, prelude::*};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use bevy_rapier2d::prelude::*;

use crate::{state::{AllowedState, GameState}, util::Cooldown};

use super::{animations::AnimalState, dog::DogTag, physics::MoveTo, AnimalBehavior};

pub struct SheepBehaviorPlugin;
impl Plugin for SheepBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sheep_flocking);
    }
}

#[derive(Component)]
pub struct SheepTag;

#[derive(Bundle)]
pub struct SheepBundle {
    pub body: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub scene: Handle<Scene>,
    pub gltf: Handle<Gltf>,
    pub state: AnimalState,
    pub sheep_tag: SheepTag,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub impuls: ExternalImpulse,
    pub global_transform: GlobalTransform,
    pub name: Name,
    pub mass: ColliderMassProperties,
    pub allowed_game_states: AllowedState,
}

impl Default for SheepBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::Dynamic,
            collider: Collider::ball(2.),
            velocity: Velocity::default(),
            scene: Handle::default(),
            gltf: Handle::default(),
            state: AnimalState::Idle,
            name: Name::new("sheep"),
            sheep_tag: SheepTag,
            impuls: ExternalImpulse::default(),
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::HIDDEN,
            view_visibility: ViewVisibility::HIDDEN,
            transform: Transform::IDENTITY,
            global_transform: GlobalTransform::IDENTITY,
            mass: ColliderMassProperties::Mass(10.),
            allowed_game_states: AllowedState::new(GameState::Game),
        }
    }
}
fn sheep_flocking(
    mut velocities: Query<&mut Velocity>,
    dogs: Query<(Entity), With<DogTag>>,
    sheeps: Query<(Entity), ( With<SheepTag>, Without<Cooldown> )>,
    move_to: Query<&MoveTo>,
    positions: Query<&Transform>,
    rapier_context: Res<RapierContext>,
    sheep_behavior: Res<AnimalBehavior>,
) {
    sheeps.iter().for_each(|(entity)| {
        let transform = positions.get(entity).unwrap();
        let collider = Collider::ball(sheep_behavior.vision);
        let mut sheeps_in_range = Vec::new();

        rapier_context.intersections_with_shape(
            transform.translation.truncate(),
            Rot::default(),
            &collider,
            QueryFilter::default().predicate(&|ent| ent != entity && sheeps.get(ent).is_ok()),
            |ent| {
                sheeps_in_range.push(ent);
                return true;
            },
        );

        let mut acc_direction = Vec2::ZERO;

        if sheeps_in_range.len() > 0 {
            let average_position = sheeps_in_range
                .iter()
                .map(|ent| positions.get(*ent).unwrap().translation.truncate())
                .sum::<Vec2>()
                / sheeps_in_range.len() as f32;

            let cohesion =
                (average_position - transform.translation.truncate()).normalize_or_zero();
            acc_direction += cohesion * sheep_behavior.cohesion;

            let alignment = sheeps_in_range
                .iter()
                .map(|ent| velocities.get(*ent).unwrap().linvel.normalize_or_zero())
                .sum::<Vec2>()
                / sheeps_in_range.len() as f32;
            acc_direction += alignment * sheep_behavior.alignment;

            let separation = (sheeps_in_range
                .iter()
                .map(|ent| {
                    let position = positions.get(*ent).unwrap().translation.truncate();
                    let distance = position.distance(transform.translation.truncate());
                    let direction = (transform.translation.truncate() - position);
                    direction / distance
                })
                .sum::<Vec2>()
                / sheeps_in_range.len() as f32)
                .normalize_or_zero();
            acc_direction += separation * sheep_behavior.separation;
        }

        let flee = (dogs
            .iter()
            .filter_map(|ent| {
                let position = positions.get(ent).unwrap().translation.truncate();
                let distance = position.distance(transform.translation.truncate());

                if distance > sheep_behavior.vision * 8. {
                    return None;
                }

                let direction = (transform.translation.truncate() - position);
                Some(direction / distance)
            })
            .sum::<Vec2>()
            / dogs.iter().len() as f32);

        acc_direction += flee.normalize_or_zero() * sheep_behavior.fear;

        let mut velocity = velocities.get_mut(entity).unwrap();
        acc_direction += velocity.linvel.normalize_or_zero();

        if let Ok(move_to) = move_to.get(entity) {
            let direction =
                (move_to.postion() - transform.translation.truncate()).normalize_or_zero();
            acc_direction += direction * sheep_behavior.motivation;
        }

        velocity.linvel = acc_direction.normalize_or_zero()
            * (velocity.linvel.length() + flee.length()).min(sheep_behavior.speed);
    });
}
