use std::time::Duration;

use bevy::prelude::*;
use bevy_nine_slice_ui::{NineSliceMaterialBundle, NineSliceTexture};
use bevy_tweening::{lens::*, *};

use crate::state::{GameState, AllowedState};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menue);
        app.add_systems(Update, (hover_effect, start_game));
    }
}

#[derive(Component)]
pub struct PlayButton;

fn start_game(
    query: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
    mut states: ResMut<NextState<GameState>>,
) {
    query.iter().for_each(|interaction| match *interaction {
        Interaction::Pressed => {
            info!("starting game");
            states.set(GameState::Game);
        }
        _ => {}
    });
}

fn hover_effect(
    mut cmd: Commands,
    query: Query<(Entity, &Interaction)>,
    animated: Query<Entity, With<Animator<Transform>>>,
) {
    query
        .iter()
        .for_each(|(ent, interaction)| match interaction {
            Interaction::Hovered => {
                if animated.get(ent).is_ok() {
                    return;
                }
                let tween = Tween::new(
                    EaseFunction::SineInOut,
                    Duration::from_millis(500),
                    TransformScaleLens {
                        start: Vec3::ONE,
                        end: Vec3::ONE * 1.1,
                    },
                )
                .with_repeat_count(RepeatCount::Infinite)
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
                cmd.entity(ent).insert(Animator::new(tween));
            }
            Interaction::None => {
                if animated.get(ent).is_err() {
                    return;
                }

                cmd.entity(ent).remove::<Animator<Transform>>();
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
                width: Val::Px(300.),
                height: Val::Px(300.),
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
                        font_size: 20.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(100.),
                    height: Val::Px(50.),
                    padding: UiRect::bottom(Val::Px(50.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });

            cmd.spawn(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(100.),
                    height: Val::Px(50.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .insert(PlayButton)
            .insert(NineSliceTexture::from_slice(
                server.load("sprites/ui.png"),
                Rect::new(48., 0., 96., 48.),
            ))
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Play",
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
