use bevy::{
    asset::AssetMetaCheck,
    audio::{PlaybackMode, Volume, VolumeLevel},
};
#[allow(unused)]
use bevy::{
    gltf::Gltf, prelude::*, render::texture::ImageSamplerDescriptor, window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

#[cfg(not(debug_assertions))]
use bevy_embedded_assets::EmbeddedAssetPlugin;

use bevy_nine_slice_ui::NineSliceUiPlugin;
use level::Levels;
use settings::GameSettings;
use state::GameAssets;

mod animals;
mod camera;
mod controls;
mod credits;
mod goal;
mod level;
mod menu;
mod settings;
mod state;
mod trap;
mod ui;
mod util;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            #[cfg(not(debug_assertions))]
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas : Some("#canvas".into()),
                        fit_canvas_to_parent : true,
                        title: "A strange shepherd's quest".into(),
                        ..default()
                    }),
                    ..default()
                }),
            #[cfg(not(debug_assertions))]
            bevy_egui::EguiPlugin,
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::default(),
            RapierPhysicsPlugin::<()>::default(),
            NineSliceUiPlugin::default(),
            controls::ControlPlugin,
            animals::SheepPlugin,
            bevy_tweening::TweeningPlugin,
            bevy_aseprite::AsepritePlugin,
            menu::MenuPlugin,
            state::StatePlugin,
            camera::CameraPlugin,
            util::UtilPlugin,
            level::LevelPlugin,
        ))
        .add_plugins((
            settings::SettingsPlugin,
            trap::TrapPlugin,
            goal::GoalPlugin,
            ui::UiPlugin,
            credits::CreditsPlugin,
        ))
        .add_systems(Startup, load)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(AmbientLight {
            color: Color::rgb_u8(194, 229, 156),
            brightness: 0.2,
        })
        .insert_resource(GameSettings::default())
        .add_systems(Startup, background_music)
        .run();
}

#[derive(Component)]
pub struct BackgroundMusic;

fn background_music(mut cmd: Commands, server: Res<AssetServer>, volume: Res<GameSettings>) {
    cmd.spawn(AudioBundle {
        source: server.load("audio/forest.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Absolute(VolumeLevel::new(volume.music)),
            ..default()
        },
    })
    .insert(BackgroundMusic);
}

fn load(mut cmd: Commands, mut game_assets: ResMut<GameAssets>, server: Res<AssetServer>) {
    let dog_handle: Handle<Gltf> = server.load("models/pug.glb");
    let sheep_handle: Handle<Gltf> = server.load("models/sheep.glb");
    let llama_handle: Handle<Gltf> = server.load("models/llama.glb");

    game_assets.add(sheep_handle.clone().untyped());
    game_assets.add(dog_handle.clone().untyped());
    game_assets.add(llama_handle.clone().untyped());
    game_assets.add(server.load_folder("audio").untyped());

    cmd.insert_resource(Levels::new(vec![
        server.load("levels/1.level.ron"),
        server.load("levels/2.level.ron"),
        server.load("levels/3.level.ron"),
        server.load("levels/4.level.ron"),
    ]));
}
