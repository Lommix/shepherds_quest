use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::Velocity,
    geometry::{Collider, Sensor},
    plugin::RapierContext,
};

use crate::{
    animals::{animations::AnimalState, sheep::SheepTag},
    util::LifeTime,
};

use super::{
    goal::{FAIL_GLOW, GLOW_MESH},
    Score,
};
pub struct TrapPlugin;
impl Plugin for TrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (watch_trap_enter, spawn_trap));
    }
}

#[derive(Component)]
pub struct Trap {
    size: Vec2,
}
impl Trap {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

#[derive(Bundle)]
pub struct TrapBundle {
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub trap: Trap,
    pub name: Name,
}

impl Default for TrapBundle {
    fn default() -> Self {
        Self {
            trap: Trap::new(Vec2::new(10., 10.)),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            name: Name::new("trap"),
        }
    }
}

fn spawn_trap(
    query: Query<(Entity, &Trap), Without<Handle<Mesh>>>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.iter().for_each(|(entity, goal)| {
        let mesh = meshes.add(Mesh::from(shape::Quad::new(goal.size)));
        let mut material: StandardMaterial = (Color::RED.with_a(0.3)).into();
        material.unlit = true;
        material.alpha_mode = AlphaMode::Blend;

        cmd.entity(entity)
            .insert(mesh)
            .insert(Sensor)
            .insert(Collider::cuboid(goal.size.x / 2., goal.size.y / 2.))
            .insert(materials.add(material));
    });
}

fn watch_trap_enter(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    goals: Query<Entity, With<Trap>>,
    sheeps: Query<(Entity), With<SheepTag>>,
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
                    .insert(LifeTime::from_seconds(0.5))
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
