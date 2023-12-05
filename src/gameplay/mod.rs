use bevy::prelude::*;

use crate::{
    animals::sheep::SheepTag,
    level::{loader::LevelAsset, LevelLoaded},
    state::GameState,
};

pub mod goal;
pub mod trap;
pub mod ui;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((trap::TrapPlugin, goal::GoalPlugin, ui::UiPlugin));
        app.init_resource::<Score>();
        app.add_systems(Update, check_progress.run_if(in_state(GameState::Game)));
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub lost: usize,
    pub saved: usize,
}

fn reset_score(mut score: ResMut<Score>) {
    score.lost = 0;
    score.saved = 0;
}

fn check_progress(
    score: Res<Score>,
    level: Query<&Handle<LevelAsset>, With<LevelLoaded>>,
    sheeps: Query<With<SheepTag>>,
    mut dialog: Query<&mut Text, With<ui::Dialog>>,
    mut state: ResMut<NextState<GameState>>,
    levels: Res<Assets<LevelAsset>>,
) {
    if sheeps.iter().count() > 0 {
        return;
    }

    let Ok(mut dialog) = dialog.get_single_mut() else {
        return;
    };

    let Ok(Some(level)) = level.get_single().map(|l| levels.get(l)) else {
        return;
    };

    //calculate score
    let total = score.lost + score.saved;
    let percent = score.saved as f32 / total as f32 * 100.;

    if percent > 50. {
        dialog.sections[0].value = format!(
            "{}. You escorted {} of the sheeps to safty!",
            level.win, percent
        );
    } else {
        dialog.sections[0].value = format!(
            "{}. You only escorted {} of the sheeps to safty! Try again!",
            level.loose, percent
        );
    }
}
