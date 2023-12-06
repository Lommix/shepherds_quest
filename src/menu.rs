#![allow(unused)]

use crate::state::{AllowedState, GameState};
use bevy::{
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
};
use bevy_aseprite::AsepriteBundle;
use bevy_nine_slice_ui::NineSliceUiTexture;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menue);
        app.add_systems(Update, hover_effect);
    }
}

#[derive(Component)]
pub struct LevelSelectorButton(pub usize);

fn hover_effect(_cmd: Commands, mut query: Query<(Entity, &Interaction, &mut NineSliceUiTexture)>) {
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

fn spawn_menue(mut cmd: Commands, server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
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
            .insert(LevelSelectorButton(0))
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
            .insert(LevelSelectorButton(0))
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
        });
    });
}
