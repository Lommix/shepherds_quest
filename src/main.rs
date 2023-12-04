#![allow(unused)]

use bevy::{
    gltf::Gltf, prelude::*, render::texture::ImageSamplerDescriptor, window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

#[cfg(not(debug_assertions))]
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_nine_slice_ui::NineSlicePlugin;
use state::GameAssets;

mod animals;
mod camera;
mod controls;
mod debug;
mod gameplay;
mod level;
mod menu;
mod ron_loader;
mod state;
mod util;

fn main() {
    App::new()
        .add_plugins((
            #[cfg(not(debug_assertions))]
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            #[cfg(debug_assertions)]
            debug::DebugPlugin,
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
                        title: "Bevy App".to_string(),
                        resolution: WindowResolution::new(1200., 900.),
                        ..default()
                    }),
                    ..default()
                }),
            WorldInspectorPlugin::default(),
            RapierPhysicsPlugin::<()>::default(),
            // RapierDebugRenderPlugin::default(),
            NineSlicePlugin::default(),
            controls::ControlPlugin,
            animals::SheepPlugin,
            bevy_tweening::TweeningPlugin,
            bevy_aseprite::AsepritePlugin,
            menu::MenuPlugin,
            state::StatePlugin,
            camera::CameraPlugin,
            gameplay::GameplayPlugin,
            util::UtilPlugin,
            level::LevelPlugin,
        ))
        .add_systems(Startup, load_models)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .run();
}

fn load_models(mut game_assets: ResMut<GameAssets>, server: Res<AssetServer>) {
    let dog_handle: Handle<Gltf> = server.load("models/pug.glb");
    let sheep_handle: Handle<Gltf> = server.load("models/sheep.glb");
    let llama_handle: Handle<Gltf> = server.load("models/sheep.glb");

    game_assets.add(sheep_handle.clone().untyped());
    game_assets.add(dog_handle.clone().untyped());
    game_assets.add(llama_handle.clone().untyped());
}
