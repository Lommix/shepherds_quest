use bevy::prelude::*;
use bevy_nine_slice_ui::NineSliceUiTexture;

use crate::{
    menu::LevelSelectorButton,
    state::{AllowedState, GameState},
    ui::{DialogBoxTag, CreditsButton},
};

use super::{
    builder::LoadLevelEvent,
    loader::LevelAsset,
    progress::{LevelLost, LevelWon},
    Levels,
};
pub struct LevelTransitionPlugin;
impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (retry_level, next_level, level_select_button));
        app.add_systems(OnEnter(GameState::Prepare), prepare_next_level);
    }
}

fn prepare_next_level(mut events: EventWriter<LoadLevelEvent>, current_level: Res<Levels>) {
    events.send(LoadLevelEvent::new(current_level.current()));
}

fn level_select_button(
    mut state: ResMut<NextState<GameState>>,
    mut current_level: ResMut<Levels>,
    query: Query<(&Interaction, &LevelSelectorButton), Changed<Interaction>>,
) {
    query
        .iter()
        .for_each(|(interaction, selection)| match *interaction {
            Interaction::Pressed => {
                current_level.set(selection.0.clone());
                state.set(GameState::Prepare);
            }
            _ => {}
        });
}

fn retry_level(
    mut event: EventReader<LevelLost>,
    mut cmd: Commands,
    mut dialog_box: Query<&mut Visibility, With<DialogBoxTag>>,
    current_level: Res<Levels>,
    server: Res<AssetServer>,
) {
    if event.iter().next().is_none() {
        return;
    }
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
    .insert(AllowedState::new(GameState::Game))
    .with_children(|cmd| {
        spawn_progress_button("retry", current_level.current(), cmd, &server);
    });

    dialog_box.iter_mut().for_each(|mut vis| {
        *vis = Visibility::Visible;
    });
}

fn next_level(
    mut event: EventReader<LevelWon>,
    mut cmd: Commands,
    mut dialog_box: Query<&mut Visibility, With<DialogBoxTag>>,
    levels: Res<Levels>,
    server: Res<AssetServer>,
) {
    if event.iter().next().is_none() {
        return;
    }

    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .insert(AllowedState::new(GameState::Game))
    .with_children(|cmd| {
        if let Some(next) = levels.next() {
            spawn_progress_button("Next Level", next, cmd, &server);
        }
        if levels.is_last() {
            cmd.spawn(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(200.),
                    height: Val::Px(50.),
                    margin: UiRect::vertical(Val::Px(15.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .insert(CreditsButton)
            .insert(NineSliceUiTexture::from_slice(
                server.load("sprites/ui.png"),
                Rect::new(48., 0., 96., 48.),
            ))
            .with_children(|cmd| {
                cmd.spawn(TextBundle {
                    text: Text::from_section(
                        "Happy end!",
                        TextStyle {
                            font_size: 20.,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });
            return;
        }

        spawn_progress_button("Retry", levels.current(), cmd, &server);
    });

    dialog_box.iter_mut().for_each(|mut vis| {
        *vis = Visibility::Visible;
    });
}

fn spawn_progress_button(
    text: &str,
    level: Handle<LevelAsset>,
    cmd: &mut ChildBuilder,
    server: &Res<AssetServer>,
) -> Entity {
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
    .insert(LevelSelectorButton(level))
    .insert(NineSliceUiTexture::from_slice(
        server.load("sprites/ui.png"),
        Rect::new(48., 0., 96., 48.),
    ))
    .with_children(|cmd| {
        cmd.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: 20.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        });
    })
    .id()
}
