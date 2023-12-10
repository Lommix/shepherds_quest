use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    input::{mouse::MouseButtonInput, touch::TouchPhase},
    prelude::*,
};

use crate::{
    animals::{dog::DogTag, physics::MoveTo},
    camera::MainCamera,
    GameSettings,
};

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapClickEvent>();
        app.add_systems(Update, touch_event.run_if(on_event::<TouchInput>()));
        app.add_systems(Update, click_event.run_if(on_event::<MouseButtonInput>()));
        app.add_systems(Update, command_dog.run_if(on_event::<MapClickEvent>()));
    }
}

#[derive(Event)]
pub struct MapClickEvent {
    translation: Vec3,
    button: MouseButton,
}
impl MapClickEvent {
    pub fn translation(&self) -> Vec3 {
        self.translation
    }
    pub fn button(&self) -> MouseButton {
        self.button
    }
}

fn touch_event(
    mut touch_event: EventReader<TouchInput>,
    mut events: EventWriter<MapClickEvent>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((cam, cam_trans)) = camera.get_single() else {
        return;
    };

    touch_event.read().for_each(|event| {
        if event.phase != TouchPhase::Started {
            return;
        }

        let Some(ray) = cam.viewport_to_world(cam_trans, event.position) else {
            return;
        };

        if let Some(pos) = intersect_ray_with_z_zero(ray.origin, ray.direction) {
            events.send(MapClickEvent {
                translation: pos,
                button: MouseButton::Right,
            });
        }
    });
}

fn click_event(
    mut mouse_click: EventReader<MouseButtonInput>,
    mut events: EventWriter<MapClickEvent>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window>,
) {
    let Ok((cam, cam_trans)) = camera.get_single() else {
        return;
    };

    mouse_click.read().for_each(|event| {
        let Ok(window) = windows.get(event.window) else {
            return;
        };

        let Some(position) = window.cursor_position() else {
            return;
        };

        let Some(ray) = cam.viewport_to_world(cam_trans, position) else {
            return;
        };

        if let Some(pos) = intersect_ray_with_z_zero(ray.origin, ray.direction) {
            events.send(MapClickEvent {
                translation: pos,
                button: event.button,
            });
        }
    });
}

fn intersect_ray_with_z_zero(origin: Vec3, direction: Vec3) -> Option<Vec3> {
    if direction.z == 0.0 {
        // The ray is parallel to the plane, check if it lies on the plane
        if origin.z == 0.0 {
            Some(Vec3::new(origin.x, origin.y, 0.0))
        } else {
            None
        }
    } else {
        let t = -origin.z / direction.z;
        let x = origin.x + t * direction.x;
        let y = origin.y + t * direction.y;
        Some(Vec3::new(x, y, 0.0))
    }
}

const DOG_SOUNDS: [&str; 2] = ["audio/dog_1.ogg", "audio/dog_2.ogg"];

#[derive(Component)]
pub struct DogSound;

fn command_dog(
    mut cmd: Commands,
    mut dogs: Query<(Entity, Option<&mut MoveTo>), With<DogTag>>,
    mut click_events: EventReader<MapClickEvent>,
    dog_sounds: Query<With<DogSound>>,
    server: Res<AssetServer>,
    volume: Res<GameSettings>,
) {
    click_events.read().for_each(|event| {
        dogs.iter_mut().for_each(|(ent, move_to)| {
            if event.button() == MouseButton::Right {
                if let Some(mut move_to) = move_to {
                    move_to.set(event.translation().truncate());
                } else {
                    cmd.entity(ent)
                        .insert(MoveTo::new(event.translation().truncate()));
                }

                let random = rand::random::<usize>() % DOG_SOUNDS.len();

                if dog_sounds.iter().count() > 0 {
                    return;
                }

                cmd.spawn(AudioBundle {
                    source: server.load(DOG_SOUNDS[random]),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        volume: Volume::Absolute(VolumeLevel::new(volume.effects * 2.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(DogSound);
            }
        });
    });
}
