use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_nine_slice_ui::{NineSliceMaterialBundle, NineSliceTexture};

use crate::{
    game_start::{Level, Score},
    state::GameState,
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_ui);
        app.add_systems(
            Update,
            update_ui.run_if(on_timer(Duration::from_millis(100))),
        );
    }
}

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct ScoreText;

fn update_ui(
    mut texts: Query<&mut Text>,
    mut score: Query<Entity, With<ScoreText>>,
    mut level: Query<Entity, With<LevelText>>,
    game_level: Res<Level>,
    game_score: Res<Score>,
) {
    score.iter_mut().for_each(|ent| {
        let mut text = texts.get_mut(ent).unwrap();
        text.sections[0].value = format!("Score: {}", game_score.0);
    });

    level.iter_mut().for_each(|ent| {
        let mut text = texts.get_mut(ent).unwrap();
        text.sections[0].value = format!("Level: {}", game_level.0);
    });
}

fn spawn_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    })
    .with_children(|cmd| {
        cmd.spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Px(400.),
                height: Val::Px(50.),
                grid_template_rows: vec![GridTrack::auto(), GridTrack::auto()],
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(NineSliceMaterialBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(150.),
                    height: Val::Px(50.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                nine_slice_texture: NineSliceTexture::from_slice(
                    asset_server.load("sprites/ui.png"),
                    Rect::new(0., 0., 48., 48.),
                ),
                ..default()
            })
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Nil",
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

            cmd.spawn(NineSliceMaterialBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(150.),
                    height: Val::Px(50.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                nine_slice_texture: NineSliceTexture::from_slice(
                    asset_server.load("sprites/ui.png"),
                    Rect::new(0., 0., 48., 48.),
                ),
                ..default()
            })
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Nil",
                        TextStyle {
                            font_size: 20.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(LevelText);
            });
        });
    });
}
