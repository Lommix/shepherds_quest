use bevy::asset::AssetMetaCheck;
#[allow(unused)]
use bevy::{
    gltf::Gltf, prelude::*, render::texture::ImageSamplerDescriptor, window::WindowResolution,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

#[cfg(not(debug_assertions))]
use bevy_embedded_assets::EmbeddedAssetPlugin;

use bevy_nine_slice_ui::NineSliceUiPlugin;
use state::GameAssets;

mod animals;
mod camera;
mod controls;
mod debug;
mod goal;
mod level;
mod puls_material;
mod menu;
mod state;
mod trap;
mod ui;
mod util;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            //            #[cfg(not(debug_assertions))]
            //            EmbeddedAssetPlugin {
            //                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            //            },
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
                        resolution: WindowResolution::new(1400., 900.),
                        ..default()
                    }),
                    ..default()
                }),
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
            puls_material::LiquidMaterialsPlugin,
            trap::TrapPlugin,
            goal::GoalPlugin,
            ui::UiPlugin,
        ))
        .add_systems(Startup, load_models)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(AmbientLight {
            color: Color::rgb_u8(194, 229, 156),
            brightness: 0.2,
        })
        .run();
}

fn load_models(mut game_assets: ResMut<GameAssets>, server: Res<AssetServer>) {
    let dog_handle: Handle<Gltf> = server.load("models/pug.glb");
    let sheep_handle: Handle<Gltf> = server.load("models/sheep.glb");
    let llama_handle: Handle<Gltf> = server.load("models/llama.glb");

    game_assets.add(sheep_handle.clone().untyped());
    game_assets.add(dog_handle.clone().untyped());
    game_assets.add(llama_handle.clone().untyped());
}
