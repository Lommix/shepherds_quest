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
    levels: Res<Assets<LevelAsset>>,
    mut cmd: Commands,
    mut dialog: Query<&mut Text, With<Dialog>>,
    _state: ResMut<NextState<GameState>>,
    mut win: EventWriter<LevelWon>,
    mut loose: EventWriter<LevelLost>,
) {

    let Ok(mut dialog) = dialog.get_single_mut() else {
        return;
    };

    let Ok((entity, handle)) = level.get_single() else {
        return;
    };

    let Some(level) = levels.get(handle) else {
        return;
    };

    let lost_percent = ( score.lost as f32 / score.total_sheep as f32 ) * 100.;
    let saved_percent = ( score.saved as f32 / score.total_sheep as f32 ) * 100.;

    if saved_percent > level.win_percent.clamp(0., 100.) {
        dialog.sections[0].value = format!(
            "{} You escorted over {:.0} % of the sheeps to safty!",
            level.win, saved_percent
        );
        win.send(LevelWon);
        cmd.entity(entity).insert(LevelOver);
    }

    if lost_percent > 100. - level.win_percent.clamp(0., 100.){
        dialog.sections[0].value = format!(
            "{} You lost more than {:.0} % of the sheeps! Try again!",
            level.loose, lost_percent
        );
        loose.send(LevelLost);
        cmd.entity(entity).insert(LevelOver);
    }

}
