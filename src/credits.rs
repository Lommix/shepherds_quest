use std::time::Duration;

use bevy::prelude::*;
use bevy_nine_slice_ui::{NineSliceUiMaterialBundle, NineSliceUiTexture};
use bevy_tweening::{
    Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};

use crate::{
    state::{AllowedState, GameState},
    ui::BackToMenuButton,
};

pub struct CreditsPlugin;
impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Credits), spawn_credits);
    }
}

fn spawn_credits(mut cmd: Commands, server: Res<AssetServer>) {
    let pos_tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_millis(1000),
        bevy_tweening::lens::UiPositionLens {
            start: UiRect::vertical(Val::Px(-50.)),
            end: UiRect::vertical(Val::Px(-70.)),
        },
    )
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
    .with_repeat_count(RepeatCount::Infinite);

    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .insert(AllowedState::new(GameState::Credits))
    .with_children(|cmd| {
        cmd.spawn(ImageBundle {
            image: UiImage::new(server.load("sprites/background.png")),
            ..default()
        });
    });

    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .insert(AllowedState::new(GameState::Credits))
    .with_children(|cmd| {
        cmd.spawn(ImageBundle {
            image: UiImage::new(server.load("sprites/credits.png")),
            ..default()
        })
        .insert(Animator::new(pos_tween));
    });

    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(95.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    })
    .insert(AllowedState::new(GameState::Credits))
    .with_children(|cmd| {
        cmd.spawn(NineSliceUiMaterialBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(100.),
                height: Val::Px(180.),
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
        .with_children(|cmd| {


        cmd.spawn(TextBundle {
            style: Style {
                ..default()
            },
            text: Text::from_section(
                "And so, it was with his trusty companions by his side, that Henk the pug embarked on a journey through the cosmos, plundering sheep and other animals across countless galaxies for as long as he lived. Though he may not have fully comprehended the nature of his actions, one thing was certain - Henk's dream had come true, and he lived happily ever after.",
                TextStyle {
                    font_size: 18.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        });

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
            .insert(BackToMenuButton)
            .insert(NineSliceUiTexture::from_slice(
                server.load("sprites/ui.png"),
                Rect::new(48., 0., 96., 48.),
            ))
            .insert(AllowedState::new(GameState::Credits))
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Back to Menu",
                        TextStyle {
                            font_size: 24.,
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
