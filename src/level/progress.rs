use bevy::prelude::*;

use crate::{animals::sheep::SheepTag, state::GameState, ui::Dialog};

use super::{loader::LevelAsset, LevelLoaded, Score};

pub struct LevelProgressPlugin;
impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelWon>();
        app.add_event::<LevelLost>();
        app.add_systems(Update, check_progress.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct LevelOver;

#[derive(Event)]
pub struct LevelWon;

#[derive(Event)]
pub struct LevelLost;

fn check_progress(
    score: Res<Score>,
    level: Query<(Entity, &Handle<LevelAsset>), (With<LevelLoaded>, Without<LevelOver>)>,
    sheeps: Query<With<SheepTag>>,
    levels: Res<Assets<LevelAsset>>,
    mut cmd: Commands,
    mut dialog: Query<&mut Text, With<Dialog>>,
    _state: ResMut<NextState<GameState>>,
    mut win: EventWriter<LevelWon>,
    mut loose: EventWriter<LevelLost>,
) {
    if sheeps.iter().count() > 0 {
        return;
    }

    let Ok(mut dialog) = dialog.get_single_mut() else {
        return;
    };

    let Ok((entity, handle)) = level.get_single() else {
        return;
    };

    let Some(level) = levels.get(handle) else {
        return;
    };

    //calculate score
    let total = score.lost + score.saved;
    let percent = score.saved as f32 / total as f32 * 100.;

    if percent > level.win_percent.clamp(0., 100.) {
        dialog.sections[0].value = format!(
            "{} You escorted {:.0} % of the sheeps to safty!",
            level.win, percent
        );
        win.send(LevelWon);
    } else {
        dialog.sections[0].value = format!(
            "{} You only escorted {:.0} % of the sheeps to safty! Try again!",
            level.loose, percent
        );
        loose.send(LevelLost);
    }

    cmd.entity(entity).insert(LevelOver);
}
