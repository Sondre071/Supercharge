use super::layout;
use super::state::{OpenRouterAction, OpenRouterUi, Screen};
use eframe::egui;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    layout::with_layout(ui_state, ctx, |ui, state| draw_content(ui, state, ctx))
}

fn draw_content(
    ui: &mut egui::Ui,
    ui_state: &mut OpenRouterUi,
    ctx: &egui::Context,
) -> OpenRouterAction {
    ui.vertical_centered(|ui| {
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
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(300.0, 600.0)));

            return OpenRouterAction::GoBack;
        }

        OpenRouterAction::None
    })
    .inner
}
