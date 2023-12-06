use std::time::Duration;

use bevy::{gltf::Gltf, pbr::ExtendedMaterial, prelude::*};
use bevy_rapier2d::{
    dynamics::ExternalImpulse, geometry::Collider, pipeline::QueryFilter, plugin::RapierContext,
};
use bevy_tweening::{
    lens::TransformPositionLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};

use crate::{
    level::{loader::LevelAsset, CurrentLevel},
    state::{AllowedState, GameState},
    util::Cooldown,
};

const LLAMA_RANGE: f32 = 20.;
const LLAMA_STOMP_COOLDOWN: f32 = 2.;
const LLAMA_STOMP_FORCE: f32 = 1000.;

use super::{
    animations::AnimalState,
    sheep::SheepTag,
    telegraph::{TelegraphBundle, TelegraphMaterial, TelegraphTag},
};
pub struct LlamaPlugin;
impl Plugin for LlamaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (llama_stomp, add_telegraph_to_llama, update_progress),
        );
    }
}

#[derive(Component)]
pub struct LLamaTag;

#[derive(Bundle)]
pub struct LLamaBundle {
    pub scene: Handle<Scene>,
    pub gltf: Handle<Gltf>,
    pub llama_tag: LLamaTag,
    pub state: AnimalState,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub name: Name,
    pub allowed_game_states: AllowedState,
}
impl Default for LLamaBundle {
    fn default() -> Self {
        Self {
            scene: Handle::default(),
            gltf: Handle::default(),
            llama_tag: LLamaTag,
            state: AnimalState::Idle,
            visibility: Visibility::Inherited,
            inherited_visibility: InheritedVisibility::HIDDEN,
            view_visibility: ViewVisibility::HIDDEN,
            transform: Transform::IDENTITY,
            global_transform: GlobalTransform::IDENTITY,
            name: Name::new("llama"),
            allowed_game_states: AllowedState::new(GameState::Game),
        }
    }
}

fn llama_stomp(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut AnimalState, &Children), (With<LLamaTag>, Without<Cooldown>)>,
    telegraphs: Query<With<TelegraphTag>>,
    level: Query<&Handle<LevelAsset>>,
    positions: Query<&Transform>,
    sheeps: Query<With<SheepTag>>,
    rapier_context: Res<RapierContext>,
    _current_level: Res<CurrentLevel>,
    levels: Res<Assets<LevelAsset>>,
) {
    let Ok(handle) = level.get_single() else {
        debug!("wtf you doing");
        return;
    };

    let Some(level) = levels.get(handle) else {
        return;
    };

    query
        .iter_mut()
        .for_each(|(entity, _animal_state, children)| {
            let Ok(transform) = positions.get(entity) else {
                return;
            };

            let collider = Collider::ball(LLAMA_RANGE);
            let mut sheeps_in_range = Vec::new();
            rapier_context.intersections_with_shape(
                transform.translation.truncate(),
                0.,
                &collider,
                QueryFilter::default().predicate(&|e| sheeps.get(e).is_ok()),
                |e| {
                    let Ok(sheep_transform) = positions.get(e) else {
                        return true;
                    };

                    let direction_to_sheep =
                        sheep_transform.translation.truncate() - transform.translation.truncate();

                    cmd.entity(e)
                        .insert(Cooldown::new(Duration::from_secs_f32(0.5)))
                        .insert(ExternalImpulse {
                            impulse: direction_to_sheep.normalize() * LLAMA_STOMP_FORCE,
                            ..default()
                        });

                    sheeps_in_range.push(e);
                    true
                },
            );

            // *animal_state = AnimalState::Jumping;

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                TransformPositionLens {
                    start: Vec3::ZERO,
                    end: Vec3::new(0., 0., 5.),
                },
            )
            .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
            .with_repeat_count(RepeatCount::Finite(2));

            match children
                .iter()
                .filter(|e| telegraphs.get(**e).is_err())
                .next()
            {
                Some(child) => {
                    cmd.entity(*child).insert(Animator::new(tween));
                }
                None => (),
            };

            cmd.entity(entity)
                .insert(Cooldown::new(Duration::from_secs_f32(
                    level.llama_stomp_rate + rand::random::<f32>() * 2.,
                )));
        });
}

fn add_telegraph_to_llama(
    query: Query<Entity, Added<LLamaTag>>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TelegraphMaterial>>,
) {
    query.iter().for_each(|entity| {
        let material = TelegraphMaterial {
            progress: Vec4::new(0.5, 0., 0., 0.),
            color: Color::RED,
        };

        cmd.entity(entity).with_children(|cmd| {
            cmd.spawn(TelegraphBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::splat(LLAMA_RANGE)))),
                material: materials.add(material),
                transform: Transform::from_xyz(0., 0., 0.1),
                ..default()
            });
        });
    });
}

fn update_progress(
    telegraphs: Query<(&Handle<TelegraphMaterial>, &Parent)>,
    cooldown_havers: Query<&Cooldown>,
    mut materials: ResMut<Assets<TelegraphMaterial>>,
) {
    telegraphs.iter().for_each(|(material, parent)| {
        let Ok(cooldown) = cooldown_havers.get(parent.get()) else {
            return;
        };
        let Some(material) = materials.get_mut(material) else {
            return;
        };

        material.progress.x = cooldown.progress();
    });
}
