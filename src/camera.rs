use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, zoom);
    }
}

fn spawn_camera(mut cmd: Commands) {
    let transform =
        Transform::from_translation(Vec3::new(200., 0., 500.)).looking_at(Vec3::ZERO, Vec3::Z);

    cmd.spawn(Camera3dBundle {
        transform,
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    })
    .insert(BloomSettings {
        intensity: 0.2,
        ..default()
    })
    .insert(Name::new("camera"));
}

fn zoom(mut query: Query<&mut Transform, With<Camera>>, mut wheel_events: EventReader<MouseWheel>) {
    let Some(mut transform) = query.iter_mut().next() else {
        return;
    };

    wheel_events.read().for_each(|ev| {
        transform.translation.z += ev.y * 10.;
    })
}
