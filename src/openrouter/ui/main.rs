use super::state::Screen;
use super::{OpenRouterAction, OpenRouterUi};
use eframe::egui;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    let mut action = OpenRouterAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("OpenRouter");
        ui.add_space(20.0);

        if ui.button("New Chat").clicked() {
            ui_state.go_to(Screen::NewChat);
        }

        if ui.button("Settings").clicked() {
            ui_state.go_to(Screen::Settings);
        }

        if ui.button("Back").clicked() {
            action = OpenRouterAction::GoBack;
        }
    });

    action
}
