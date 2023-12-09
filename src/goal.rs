use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};
use bevy_rapier2d::{
    dynamics::Velocity,
    geometry::{Collider, Sensor},
    plugin::RapierContext,
};

use crate::{
    animals::{animations::AnimalState, sheep::SheepTag},
    level::Score,
    util::LifeTime,
    GameSettings,
};

pub const SUCCESS_GLOW: Handle<StandardMaterial> = Handle::weak_from_u128(12561396483470153565671);
pub const FAIL_GLOW: Handle<StandardMaterial> = Handle::weak_from_u128(125613964543455646571);
pub const GLOW_MESH: Handle<Mesh> = Handle::weak_from_u128(126565623323232325651);

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, watch_goal_enter);

        let mut materials = app
            .world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let success_glow = StandardMaterial {
            base_color: [0.0, 100.0, 0.0, 1.0].into(),
            ..default()
        };

        let fail_glow = StandardMaterial {
            base_color: [100.0, 0.0, 0.0, 1.0].into(),
            ..default()
        };

        materials.insert(SUCCESS_GLOW, success_glow);
        materials.insert(FAIL_GLOW, fail_glow);

        let mut meshes = app.world.get_resource_mut::<Assets<Mesh>>().unwrap();

        let glow_mesh = Mesh::from(shape::Quad::new(Vec2::new(1., 1.)));
        meshes.insert(GLOW_MESH, glow_mesh);
    }
}

#[derive(Component)]
pub struct GoalTag;

#[derive(Component)]
pub struct GoalSound;

#[derive(Bundle)]
pub struct GoalBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub goal: GoalTag,
    pub name: Name,
    pub sensor: Sensor,
    pub collider: Collider,
}

impl Default for GoalBundle {
    fn default() -> Self {
        Self {
            mesh: Handle::default(),
            material: Handle::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            name: Name::new("goal"),
            goal: GoalTag,
            sensor: Sensor::default(),
            collider: Collider::default(),
        }
    }
}

fn watch_goal_enter(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    _meshes: ResMut<Assets<Mesh>>,
    goals: Query<Entity, With<GoalTag>>,
    sheeps: Query<Entity, With<SheepTag>>,
    rapier_context: Res<RapierContext>,
    server: Res<AssetServer>,
    sheep_sound: Query<With<GoalSound>>,
    volume: Res<GameSettings>,
) {
    goals.iter().for_each(|entity| {
        rapier_context
            .intersections_with(entity)
            .for_each(|(a, b, _)| {
                let sheep_ent = if a == entity { b } else { a };
                if sheeps.get(sheep_ent).is_err() {
                    return;
                }

                cmd.entity(sheep_ent)
                    .insert(LifeTime::new(Duration::from_millis(200)))
                    .insert(AnimalState::Jumping)
                    .remove::<Velocity>()
                    .remove::<SheepTag>()
                    .with_children(|cmd| {
                        cmd.spawn(MaterialMeshBundle {
                            mesh: GLOW_MESH.clone(),
                            material: SUCCESS_GLOW.clone(),
                            ..default()
                        });
                    });

                score.saved += 1;

                if sheep_sound.iter().count() > 2 {
                    return;
                }

                cmd.spawn(AudioBundle {
                    source: server.load("audio/sheep.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::Absolute(VolumeLevel::new(volume.effects * 0.5)),
                        ..default()
                    },
                    ..default()
                })
                .insert(GoalSound);

            })
    });
}
