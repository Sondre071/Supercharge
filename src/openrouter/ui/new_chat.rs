use super::state::Screen;
use super::{OpenRouterAction, OpenRouterUi};
use eframe::egui;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("New Chat");
        ui.add_space(20.0);

        ui.label("Chat UI goes here.");

        if ui.button("Back").clicked() {
            ui_state.go_to(Screen::Main);
        }
    });

    OpenRouterAction::None
}
