use std::time::Duration;

use bevy::prelude::*;
use bevy_nine_slice_ui::{NineSliceUiMaterialBundle, NineSliceUiTexture};
use bevy_tweening::{lens::*, *};

use crate::{
    level::LoadLevelEvent,
    state::{AllowedState, GameState},
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menue);
        app.add_systems(Update, (hover_effect, start_game));
    }
}

const CAMPAIGN_LEVELS: [&str; 4] = [
    "levels/first.level.ron",
    "levels/second.level.ron",
    "levels/third.level.ron",
    "levels/fourth.level.ron",
];

#[derive(Component)]
pub struct LevelSelectorButton(usize);

fn start_game(
    mut events: EventWriter<LoadLevelEvent>,
    query: Query<(&Interaction, &LevelSelectorButton), Changed<Interaction>>,
    server: Res<AssetServer>,
) {
    query
        .iter()
        .for_each(|(interaction, selection)| match *interaction {
            Interaction::Pressed => {
                events.send(LoadLevelEvent::new(
                    server.load(CAMPAIGN_LEVELS[selection.0]),
                ));
            }
            _ => {}
        });
}

fn hover_effect(
    mut cmd: Commands,
    mut query: Query<(Entity, &Interaction, &mut NineSliceUiTexture)>,
) {
    query
        .iter_mut()
        .for_each(|(ent, interaction, mut texture)| match interaction {
            Interaction::Hovered => {
                texture.blend_mix = 0.1;
            }
            Interaction::None => {
                texture.blend_mix = 0.;
            }
            _ => {}
        })
}

fn spawn_menue(mut cmd: Commands, server: Res<AssetServer>) {
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
                width: Val::Px(500.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // nine_slice_texture: NineSliceTexture::from_slice(
            //     server.load("sprites/ui.png"),
            //     Rect::new(0., 0., 48., 48.),
            // ),
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(TextBundle {
                text: Text::from_section(
                    "Shepherd's Quest",
                    TextStyle {
                        font_size: 40.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    display: Display::Flex,
                    width: Val::Auto,
                    height: Val::Px(50.),
                    margin: UiRect::vertical(Val::Px(20.)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });

            cmd.spawn(TextBundle {
                text: Text::from_section(
                    "Help Hank the pug to fullfill his life long dream of becoming a Shepherd's dog!",
                    TextStyle {
                        font_size: 16.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(300.),
                    height: Val::Px(50.),
                    margin: UiRect::vertical(Val::Px(20.)),
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
                    margin: UiRect::vertical(Val::Px(5.)),
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
                        "Level 1",
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
                    margin: UiRect::vertical(Val::Px(5.)),
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
