use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::Velocity,
    geometry::{Collider, Sensor},
    plugin::RapierContext,
};

use crate::{
    animals::{animations::AnimalState, sheep::SheepTag},
    level::Score,
    liquid::LiquidMaterial,
    util::LifeTime,
};

use super::goal::{FAIL_GLOW, GLOW_MESH};
pub struct TrapPlugin;
impl Plugin for TrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, watch_trap_enter);
    }
}

#[derive(Component)]
pub struct TrapTag;

#[derive(Bundle)]
pub struct TrapBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<LiquidMaterial>,
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

fn watch_trap_enter(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    _meshes: ResMut<Assets<Mesh>>,
    goals: Query<Entity, With<TrapTag>>,
    sheeps: Query<Entity, With<SheepTag>>,
    rapier_context: Res<RapierContext>,
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

                score.lost += 1;
            })
    });
}
