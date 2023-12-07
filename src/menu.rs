#![allow(unused)]

use crate::{
    level::{
        builder::LoadLevelEvent,
        loader::{LevelAsset, LevelAssetLoader},
        Levels, CAMPAIGN_LEVELS,
    },
    state::{AllowedState, GameState},
};
use bevy::{
    asset::AssetLoader,
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
    tasks::{AsyncComputeTaskPool, ComputeTaskPool, Task},
};
use bevy_aseprite::AsepriteBundle;
use bevy_nine_slice_ui::NineSliceUiTexture;
use rfd::AsyncFileDialog;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::{future::Future, sync::Mutex};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FileDialogState>();
        app.add_event::<FileLoadedEvent>();
        app.add_systems(OnEnter(GameState::Menu), spawn_menu);
        app.add_systems(
            Update,
            (hover_effect, dialog_state_checker, load_custom_level),
        );
    }
}

#[derive(Component)]
pub struct LevelSelectorButton(pub Handle<LevelAsset>);

#[derive(Component)]
pub struct LevelLoadButton;

#[derive(Event)]
pub struct FileLoadedEvent {
    pub file: String,
    pub content: Vec<u8>,
}

fn dialog_state_checker(
    mut state: ResMut<FileDialogState>,
    mut events: EventWriter<LoadLevelEvent>,
    mut level_assets: ResMut<Assets<LevelAsset>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut levels: ResMut<Levels>,
) {
    if state.loading {
        let Ok(data) = state.channel.1.try_recv() else {
            return;
        };

        info!("file loaded");
        state.loading = false;

        let Ok(asset) = LevelAsset::try_from(data.as_slice()) else {
            return;
        };

        let handle = level_assets.add(asset);
        levels.set(handle.clone());
        next_state.set(GameState::GameOver);
    }
}

#[derive(Resource)]
pub struct FileDialogState {
    loading: bool,
    channel: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
}

impl Default for FileDialogState {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            loading: false,
            channel: (tx, rx),
        }
    }
}

unsafe impl Send for FileDialogState {}
unsafe impl Sync for FileDialogState {}

fn load_custom_level(
    buttons: Query<&Interaction, With<LevelLoadButton>>,
    mut state: ResMut<FileDialogState>,
) {
    buttons.iter().for_each(|interaction| match interaction {
        Interaction::Pressed => {
            info!("clicked");

            if state.loading {
                return;
            }

            state.loading = true;
            futures_lite::future::block_on(async {
                let dialog_future = AsyncFileDialog::new()
                    .set_title("Load a custom Level form a .ron file")
                    .add_filter("ron", &["ron"])
                    .pick_file();

                let sender = state.channel.0.clone();
                let file_handle = AsyncComputeTaskPool::get()
                    .spawn(async move {
                        let dialog = AsyncFileDialog::new()
                            .set_title("Load a custom Level form a .ron file")
                            .add_filter("ron", &["ron"])
                            .pick_file();

                        let file = dialog.await.unwrap();
                        let content = file.read().await;
                        sender.send(content).unwrap();
                    })
                    .detach();
            });
        }
        _ => {}
    });
}

fn hover_effect(mut query: Query<(Entity, &Interaction, &mut NineSliceUiTexture)>) {
    query
        .iter_mut()
        .for_each(|(_ent, interaction, mut texture)| match interaction {
            Interaction::Hovered => {
                texture.blend_mix = 0.1;
            }
            Interaction::None => {
                texture.blend_mix = 0.;
            }
            _ => {}
        })
}
fn spawn_menu(mut cmd: Commands, server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 200,
        height: 200,
        ..default()
    };

    let texture_usages = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: texture_usages | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Rgba16Float],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };
    image.resize(size);
    let image_handle = images.add(image);

    cmd.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(image_handle.clone()),
            order: 2,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLUE.with_a(0.)),
        },
        ..default()
    })
    .insert(RenderLayers::layer(5));

    cmd.spawn(AsepriteBundle {
        aseprite: server.load("sprites/henk.aseprite"),
        ..default()
    })
    .insert(RenderLayers::layer(5));

    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .insert(AllowedState::new(GameState::Menu))
    .with_children(|cmd| {
        cmd.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Px(600.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(ImageBundle{
                        image: UiImage::new(image_handle),
                        ..default()
                    });
            cmd.spawn(TextBundle {
                text: Text::from_section(
                            "A strange Shepherd's Quest",
                    TextStyle {
                        font_size: 40.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    display: Display::Flex,
                    width: Val::Auto,
                    height: Val::Auto,
                    margin: UiRect::vertical(Val::Px(20.)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });

            cmd.spawn(TextBundle {
                text: Text::from_section(
                    "Help Henk the pug to fulfill his lifelong dream of becoming a shepherd's dog. For the first time, he met a strange Shepherd, that is willing to give him a chance.",
                    TextStyle {
                        font_size: 16.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(400.),
                    height: Val::Px(50.),
                    margin: UiRect::bottom(Val::Px(80.)),
                    padding : UiRect::all(Val::Px(10.)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });

            cmd.spawn(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(200.),
                    height: Val::Px(50.),
                    margin: UiRect::bottom(Val::Px(10.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .insert(LevelSelectorButton(server.load(CAMPAIGN_LEVELS[0])))
            .insert(NineSliceUiTexture::from_slice(
                server.load("sprites/ui.png"),
                Rect::new(48., 0., 96., 48.),
            ))
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Campaign",
                        TextStyle {
                            font_size: 20.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });

            cmd.spawn(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(200.),
                    height: Val::Px(50.),
                    margin: UiRect::vertical(Val::Px(10.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .insert(LevelLoadButton)
            .insert(NineSliceUiTexture::from_slice(
                server.load("sprites/ui.png"),
                Rect::new(48., 0., 96., 48.),
            ))
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Custom Level",
                        TextStyle {
                            font_size: 20.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });

            cmd.spawn(TextBundle {
                style: Style {
                            margin: UiRect::top(Val::Px(100.)),
                            ..default()
                        },
                text: Text::from_section(
                    "Click to run, Mouswheel to zoom\nbuild your own levels and share them!",
                    TextStyle {
                        font_size: 20.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            });

        });
    });
}
