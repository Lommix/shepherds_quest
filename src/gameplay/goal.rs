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

use super::Score;

pub const SUCCESS_GLOW: Handle<StandardMaterial> = Handle::weak_from_u128(12561396483470153565671);
pub const FAIL_GLOW: Handle<StandardMaterial> = Handle::weak_from_u128(125613964543455646571);
pub const GLOW_MESH: Handle<Mesh> = Handle::weak_from_u128(126565623323232325651);

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_goal, watch_goal_enter));

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
pub struct Goal {
    size: Vec2,
}
impl Goal {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

#[derive(Bundle)]
pub struct GoalBundle {
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub goal: Goal,
    pub name: Name,
}

impl Default for GoalBundle {
    fn default() -> Self {
        Self {
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            name: Name::new("goal"),
            goal: Goal {
                size: Vec2::new(50.0, 50.0),
            },
        }
    }
}

fn spawn_goal(
    query: Query<(Entity, &Goal), Without<Handle<Mesh>>>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.iter().for_each(|(entity, goal)| {
        let mesh = meshes.add(Mesh::from(shape::Quad::new(goal.size)));
        let mut material: StandardMaterial = (Color::BLUE.with_a(0.3)).into();
        material.unlit = true;
        material.alpha_mode = AlphaMode::Blend;

        cmd.entity(entity)
            .insert(mesh)
            .insert(Sensor)
            .insert(Collider::cuboid(goal.size.x / 2., goal.size.y / 2.))
            .insert(materials.add(material));
    });
}

fn watch_goal_enter(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    goals: Query<Entity, With<Goal>>,
    sheeps: Query<(Entity), With<SheepTag>>,
    rapier_context: Res<RapierContext>,
) {
    goals.iter().for_each(|entity| {
        rapier_context
            .intersections_with(entity)
            .for_each(|(_, collider, _)| {
                let Ok(sheep_ent) = sheeps.get(collider) else {
                    return;
                };

                cmd.entity(sheep_ent)
                    .insert(LifeTime::from_seconds(0.2))
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
            })
    });
}
