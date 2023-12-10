use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*, utils::HashSet,
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

use super::goal::{FAIL_GLOW, GLOW_MESH};
pub struct TrapPlugin;
impl Plugin for TrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (watch_trap_enter, update_emission));
    }
}

#[derive(Component)]
pub struct TrapTag;

#[derive(Component)]
pub struct DeathSound;

#[derive(Bundle)]
pub struct TrapBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub trap: TrapTag,
    pub name: Name,
    pub sensor: Sensor,
    pub collider: Collider,
}

impl Default for TrapBundle {
    fn default() -> Self {
        Self {
            mesh: Handle::default(),
            material: Handle::default(),
            trap: TrapTag,
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            sensor: Sensor::default(),
            collider: Collider::default(),
            name: Name::new("trap"),
        }
    }
}

fn update_emission(
    query: Query<&Handle<StandardMaterial>, With<TrapTag>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.iter().for_each(|handle| {
        let Some(material) = materials.get_mut(handle) else {
            return;
        };
        // not gonna extend the standard material shader for this, lol
        material.base_color = Color::rgb(
            time.elapsed().as_secs_f32().sin().abs() * 3. + 0.2,
            0.0,
            0.0,
        );
    });
}

fn watch_trap_enter(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    _meshes: ResMut<Assets<Mesh>>,
    goals: Query<Entity, With<TrapTag>>,
    sheeps: Query<Entity, With<SheepTag>>,
    rapier_context: Res<RapierContext>,
    server: Res<AssetServer>,
    death_sound: Query<With<DeathSound>>,
    volume: Res<GameSettings>,
) {
    let mut dying_sheeps = HashSet::new();
    goals.iter().for_each(|entity| {
        rapier_context
            .intersections_with(entity)
            .for_each(|(a, b, _)| {
                let sheep_ent = if a == entity { b } else { a };
                if sheeps.get(sheep_ent).is_err() {
                    return;
                }


                dying_sheeps.insert(sheep_ent);
                cmd.entity(sheep_ent)
                    .insert(LifeTime::new(Duration::from_millis(500)))
                    .insert(AnimalState::Dead)
                    .remove::<Velocity>()
                    .remove::<SheepTag>()
                    .with_children(|cmd| {
                        cmd.spawn(MaterialMeshBundle {
                            mesh: GLOW_MESH.clone(),
                            material: FAIL_GLOW.clone(),
                            ..default()
                        });
                    });

                if death_sound.iter().count() > 2 {
                    return;
                }

                cmd.spawn(AudioBundle {
                    source: server.load("audio/sheep_death.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::Absolute(VolumeLevel::new(volume.effects * 0.5)),
                        ..default()
                    },
                    ..default()
                })
                .insert(DeathSound);
            })
    });
    score.lost += dying_sheeps.len();
}
