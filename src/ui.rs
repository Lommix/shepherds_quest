use std::time::Duration;

use crate::{
    level::Score,
    state::{AllowedState, GameState},
    util::VisibilityTimer,
};
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
use bevy_aseprite::AsepriteBundle;
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
        app.add_systems(Update, (back_to_menu, roll_credits, close_dialog));
    }
}

#[derive(Resource, Default)]
struct PortraitRender(Handle<Image>);

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct HideDialogButton;

#[derive(Component)]
pub struct CreditsButton;

#[derive(Component)]
pub struct DialogBoxTag;

#[derive(Component)]
pub struct Dialog;

#[derive(Component)]
struct ScoreText;

fn update_ui(
    mut texts: Query<&mut Text>,
    mut level: Query<Entity, With<ScoreText>>,
    game_score: Res<Score>,
) {
    level.iter_mut().for_each(|ent| {
        let Ok(mut text) = texts.get_mut(ent) else {
            return;
        };

        let percent_lost = game_score.lost as f32 / game_score.total_sheep as f32;
        let percent_saved = game_score.saved as f32 / game_score.total_sheep as f32;

        text.sections[0].value = format!("Lost: {:.0} %  Escorted {:.0} %", percent_lost * 100., percent_saved * 100.);
    });
}

fn spawn_ui(mut cmd: Commands, portrait: Res<PortraitRender>, server: Res<AssetServer>) {
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
                cmd.spawn(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Lost 0 %",
                        TextStyle {
                            font_size: 20.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(ScoreText);
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
            //back to menu
            cmd.spawn(ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.),
                    bottom: Val::Percent(2.),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                ..default()
            })
            .insert(BackToMenuButton)
            .insert(NineSliceUiTexture::from_slice(
                server.load("sprites/ui.png"),
                Rect::new(48., 0., 96., 48.),
            ))
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Back to Menu",
                        TextStyle {
                            font_size: 16.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });
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
                    server.load("sprites/ui.png"),
                    Rect::new(0., 0., 48., 48.),
                ),
                ..default()
            })
            .insert(DialogBoxTag)
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

                //back to menu
                cmd.spawn(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(10.),
                        bottom: Val::Px(10.),
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(HideDialogButton)
                .insert(NineSliceUiTexture::from_slice(
                    server.load("sprites/ui.png"),
                    Rect::new(48., 0., 96., 48.),
                ))
                .with_children(|cmd| {
                    cmd.spawn(TextBundle {
                        text: Text::from_section(
                            "Ok",
                            TextStyle {
                                font_size: 24.,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
                //dialog
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Dialog",
                        TextStyle {
                            font_size: 20.,
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

fn close_dialog(
    buttons: Query<&Interaction, (Changed<Interaction>, With<HideDialogButton>)>,
    mut dialog: Query<&mut Visibility, With<DialogBoxTag>>,
) {
    buttons.iter().for_each(|interaction| match *interaction {
        Interaction::Pressed => {
            dialog.iter_mut().for_each(|mut visibility| {
                *visibility = Visibility::Hidden;
            });
        }
        _ => {}
    });
}

fn roll_credits(
    mut state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, &CreditsButton), Changed<Interaction>>,
) {
    query
        .iter()
        .for_each(|(interaction, _)| match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Credits);
            }
            _ => {}
        });
}

fn back_to_menu(
    mut state: ResMut<NextState<GameState>>,
    query: Query<(&Interaction, &BackToMenuButton), Changed<Interaction>>,
) {
    query
        .iter()
        .for_each(|(interaction, _)| match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Menu);
            }
            _ => {}
        });
}

fn portrait_render_scene(
    mut cmd: Commands,
    mut portrait_render: ResMut<PortraitRender>,
    server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 128,
        height: 128,
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
            clear_color: ClearColorConfig::Custom(Color::GRAY.with_a(0.)),
        },
        ..default()
    })
    .insert(PORTRAIT_LAYER);

    cmd.spawn(AsepriteBundle {
        aseprite: server.load("sprites/alien.aseprite"),
        ..default()
    })
    .insert(PORTRAIT_LAYER);
}
