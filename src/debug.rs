use bevy::prelude::*;
use bevy_egui::{egui::Align2, *};

use crate::animals::AnimalBehavior;


pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_gui);
    }
}

fn debug_gui(
    mut cmd: Commands,
    mut egui_context: EguiContexts,
    mut sheep_behavior: ResMut<AnimalBehavior>,
) {
    egui::Window::new("Sheep ai")
        .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut sheep_behavior.alignment, 0.0..=1.0).text("alignment"));
            ui.add(egui::Slider::new(&mut sheep_behavior.cohesion, 0.0..=1.0).text("cohesion"));
            ui.add(egui::Slider::new(&mut sheep_behavior.separation, 0.0..=1.0).text("separation"));
            ui.add(egui::Slider::new(&mut sheep_behavior.speed, 0.0..=300.0).text("speed"));
            ui.add(egui::Slider::new(&mut sheep_behavior.vision, 0.0..=100.0).text("vision"));
            ui.add(egui::Slider::new(&mut sheep_behavior.fear, 0.0..=5.0).text("fear"));
            ui.add(egui::Slider::new(&mut sheep_behavior.motivation, 0.0..=5.0).text("motivation"));
        });
}
