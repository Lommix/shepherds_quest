use bevy::prelude::*;

pub mod goal;
pub mod trap;
mod ui;
mod game_start;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            trap::TrapPlugin,
            game_start::GameStartPlugin,
            goal::GoalPlugin,
            ui::UiPlugin,
        ));
        app.init_resource::<Score>();
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
