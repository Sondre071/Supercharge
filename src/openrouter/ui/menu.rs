use super::state::{OpenRouterAction, OpenRouterUi, Screen};
use eframe::egui;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    let mut action = OpenRouterAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("OpenRouter");
        ui.add_space(40.0);

        if ui.button("New Chat").clicked() {
            ui_state.go_to(Screen::Chat);
        }

        ui.add_space(10.0);

        if ui.button("Settings").clicked() {
            ui_state.go_to(Screen::Settings);
        }

        ui.add_space(10.0);

        if ui.button("Exit").clicked() {
            action = OpenRouterAction::GoBack;
        }
    });

    action
}
