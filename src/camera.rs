use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::animals::dog::DogTag;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (zoom, follow_camera));
    }
}

#[derive(Component)]
struct ZoomDistance(f32);

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    })
    .insert(FogSettings {
        color: Color::WHITE,
        falloff: FogFalloff::Linear {
            start: 270.,
            end: 600.,
        },
        ..default()
    })
    .insert(ZoomDistance(100.))
    .insert(BloomSettings {
        intensity: 0.3,
        composite_mode: BloomCompositeMode::Additive,
        prefilter_settings: BloomPrefilterSettings {
            threshold: 1.0,
            threshold_softness: 0.1,
            ..default()
        },
        ..default()
    })
    .insert(MainCamera)
    .insert(Name::new("camera"));

    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}

fn zoom(mut query: Query<&mut ZoomDistance>, mut wheel_events: EventReader<MouseWheel>) {
    wheel_events.read().for_each(|ev| {
        query.iter_mut().for_each(|mut zoom| {
            zoom.0 = (zoom.0 + ev.y.clamp(-1., 1.) * 10.).clamp(20., 1000.);
        })
    })
}

fn follow_camera(
    mut camera: Query<(Entity, &ZoomDistance), With<Camera>>,
    mut postions: Query<&mut Transform>,
    dogs: Query<Entity, With<DogTag>>,
) {
    let Ok((camera, zoom)) = camera.get_single_mut() else {
        return;
    };

    let avarage_dog_position = dogs.iter().fold(Vec3::ZERO, |acc, dog| {
        let Ok(transform) = postions.get_mut(dog) else {
            return acc;
        };
        acc + transform.translation
    }) / dogs.iter().len() as f32;

    let Ok(mut cam_trans) = postions.get_mut(camera) else {
        return;
    };

    let ray = Ray {
        origin: avarage_dog_position,
        direction: Vec3::new(1.0, 1.0, 1.),
    };

    cam_trans.translation = ray.get_point(zoom.0);
    cam_trans.look_at(avarage_dog_position, Vec3::Z);
}
