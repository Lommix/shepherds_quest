use bevy::prelude::*;

mod game_start;
mod goal;
mod level;
mod spawner;
mod trap;
mod ui;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            trap::TrapPlugin,
            game_start::GameStartPlugin,
            goal::GoalPlugin,
            ui::UiPlugin,
            level::LevelPlugin,
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
