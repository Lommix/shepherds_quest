use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};

use crate::VolumeControl;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, settings);
    }
}

fn settings(mut contexts: EguiContexts, mut volume: ResMut<VolumeControl>) {
    egui::Window::new("Settings")
        .anchor(Align2::RIGHT_TOP, [1., 1.])
        .default_open(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut volume.music, 0.0..=1.0).text("Music"));
            ui.add(egui::Slider::new(&mut volume.effects, 0.0..=1.0).text("Effects"));
        });
}
