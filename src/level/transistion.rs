use bevy::prelude::*;
use bevy_nine_slice_ui::NineSliceUiTexture;

use crate::{
    menu::LevelSelectorButton,
    state::{AllowedState, GameState},
};

use super::progress::{LevelLost, LevelWon};
pub struct LevelTransitionPlugin;
impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (retry_level, next_level));
    }
}

fn retry_level(mut event: EventReader<LevelLost>, mut cmd: Commands, server: Res<AssetServer>) {
    if let None = event.iter().next() {
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
        spawn_progress_button("retry", 0, cmd, &server);
    });
}

fn next_level(mut event: EventReader<LevelWon>, mut cmd: Commands, server: Res<AssetServer>) {
    if let None = event.iter().next() {
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
    .with_children(|builder| {
        spawn_progress_button("retry", 0, builder, &server);
        spawn_progress_button("next level", 0, builder, &server);
    });
}

fn spawn_progress_button(
    text: &str,
    level: usize,
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
        Rect::new(0., 0., 48., 48.),
    ))
    .with_children(|cmd| {
        cmd.spawn(TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: 20.,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ..default()
        });
    })
    .id()
}
