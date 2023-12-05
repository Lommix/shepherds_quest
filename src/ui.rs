use std::time::Duration;

use crate::{state::{AllowedState, GameState}, level::Score};
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
    time::common_conditions::on_timer,
};
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepriteSliceBundle};
use bevy_nine_slice_ui::{NineSliceUiMaterialBundle, NineSliceUiTexture};

const PORTRAIT_LAYER: RenderLayers = RenderLayers::layer(2);

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PortraitRender>();
        app.add_systems(OnEnter(GameState::Game), spawn_ui);
        app.add_systems(Startup, portrait_render_scene);
        app.add_systems(
            Update,
            update_ui.run_if(on_timer(Duration::from_millis(100))),
        );
    }
}

#[derive(Resource, Default)]
struct PortraitRender(Handle<Image>);

#[derive(Component)]
pub struct Dialog;

#[derive(Component)]
struct LostText;

#[derive(Component)]
struct SavedText;

fn update_ui(
    mut texts: Query<&mut Text>,
    mut score: Query<Entity, With<SavedText>>,
    mut level: Query<Entity, With<LostText>>,
    game_score: Res<Score>,
) {
    score.iter_mut().for_each(|ent| {
        let mut text = texts.get_mut(ent).unwrap();
        text.sections[0].value = format!("Escorted: {}", game_score.saved);
    });

    level.iter_mut().for_each(|ent| {
        let mut text = texts.get_mut(ent).unwrap();
        text.sections[0].value = format!("Lost: {}", game_score.lost);
    });
}

fn spawn_ui(mut cmd: Commands, portrait: Res<PortraitRender>, asset_server: Res<AssetServer>) {
    info!("spawning ui");
    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .insert(AllowedState::new(GameState::Game))
    .with_children(|cmd| {
        // ----------------------------------------------------------------
        // Upper
        cmd.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(100.),
                    height: Val::Px(150.),
                    margin: UiRect::all(Val::Px(10.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            })
            .with_children(|cmd| {
                // ----------------------------------------------------------------
                // Sheep saved Panel
                cmd.spawn(NineSliceUiMaterialBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Px(130.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    nine_slice_texture: NineSliceUiTexture::from_slice(
                        asset_server.load("sprites/ui.png"),
                        Rect::new(0., 0., 48., 48.),
                    )
                    .with_blend_color(Color::GREEN)
                    .with_blend_mix(0.02),
                    ..default()
                })
                .with_children(|cmd| {
                    cmd.spawn(TextBundle {
                        text: Text::from_section(
                            "Saved 0",
                            TextStyle {
                                font_size: 15.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ..default()
                    })
                    .insert(SavedText);
                });
                // ----------------------------------------------------------------
                // Sheep lost Panel
                cmd.spawn(NineSliceUiMaterialBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Px(130.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    nine_slice_texture: NineSliceUiTexture::from_slice(
                        asset_server.load("sprites/ui.png"),
                        Rect::new(0., 0., 48., 48.),
                    )
                    .with_blend_color(Color::RED)
                    .with_blend_mix(0.01),
                    ..default()
                })
                .with_children(|cmd| {
                    cmd.spawn(TextBundle {
                        text: Text::from_section(
                            "Lost 0",
                            TextStyle {
                                font_size: 15.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ..default()
                    })
                    .insert(LostText);
                });
            });
        });

        // ----------------------------------------------------------------
        // Lower
        cmd.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.),
                height: Val::Percent(50.),
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(NineSliceUiMaterialBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(100.),
                    height: Val::Px(150.),
                    margin: UiRect {
                        left: Val::Percent(10.),
                        right: Val::Percent(10.),
                        bottom: Val::Percent(1.),
                        ..default()
                    },
                    padding: UiRect::all(Val::Percent(3.)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                nine_slice_texture: NineSliceUiTexture::from_slice(
                    asset_server.load("sprites/ui.png"),
                    Rect::new(0., 0., 48., 48.),
                ),
                ..default()
            })
            .with_children(|cmd| {
                //portrait
                cmd.spawn(ImageBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Px(100.),
                        height: Val::Px(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(portrait.0.clone()),
                    ..default()
                });

                //dialog
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Dialog",
                        TextStyle {
                            font_size: 24.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    style: Style {
                        display: Display::Flex,
                        width: Val::Percent(100.),
                        margin: UiRect::left(Val::Percent(3.)),
                        padding: UiRect::all(Val::Px(10.)),
                        height: Val::Px(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(Dialog);
            });
        });
    });
}

fn portrait_render_scene(
    mut cmd: Commands,
    mut portrait_render: ResMut<PortraitRender>,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 100,
        height: 100,
        ..default()
    };

    // This is the texture that will be rendered to.
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
    portrait_render.0 = images.add(image);

    cmd.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(portrait_render.0.clone()),
            order: 2,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLUE.with_a(0.)),
        },
        ..default()
    })
    .insert(PORTRAIT_LAYER);

    cmd.spawn(AsepriteBundle {
        aseprite: server.load("sprites/sheperd.aseprite"),
        ..default()
    })
    .insert(PORTRAIT_LAYER);
}
