use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

pub struct AnimalPhysicsPlugin;
impl Plugin for AnimalPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, face_front);
    }
}

#[derive(Component)]
pub struct MoveTo(Vec2);
impl MoveTo {
    pub fn new(position: Vec2) -> Self {
        Self(position)
    }
    pub fn set(&mut self, position: Vec2) -> &mut Self {
        self.0 = position;
        self
    }
    pub fn postion(&self) -> Vec2 {
        self.0
    }
}

fn face_front(mut query: Query<(&mut Transform, &Velocity)>) {
    query.iter_mut().for_each(|(mut transform, velocity)| {
        let direction = velocity.linvel.normalize_or_zero();
        let angle = direction.y.atan2(direction.x) + std::f32::consts::FRAC_PI_2;
        transform.rotation = Quat::from_rotation_z(angle);
    });
}
