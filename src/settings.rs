use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};

use crate::BackgroundMusic;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (settings, adjust_background_music));
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub music: f32,
    pub effects: f32,
    pub shadows: bool,
}
impl Default for GameSettings {
    fn default() -> Self {
        Self {
            music: 0.05,
            effects: 0.02,
            #[cfg(target_arch = "wasm32")]
            shadows: false,
            #[cfg(not(target_arch = "wasm32"))]
            shadows: true,
        }
    }
}

fn settings(
    mut query: Query<&mut PointLight>,
    mut contexts: EguiContexts,
    mut settings: ResMut<GameSettings>,
) {
    egui::Window::new("Settings")
        .anchor(Align2::RIGHT_TOP, [1., 1.])
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut settings.music, 0.0..=1.0).text("Music"));
            ui.add(egui::Slider::new(&mut settings.effects, 0.0..=1.0).text("Effects"));
            ui.checkbox(&mut settings.shadows, "Shadows");
        });

    query.iter_mut().for_each(|mut light| {
        light.shadows_enabled = settings.shadows;
    });
}

fn adjust_background_music(
    query: Query<&AudioSink, With<BackgroundMusic>>,
    game_settings: Res<GameSettings>,
) {
    query.iter().for_each(|settings| {
        settings.set_volume(game_settings.music);
    });
}
