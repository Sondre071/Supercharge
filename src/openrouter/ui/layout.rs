use super::state::{OpenRouterAction, OpenRouterUi};
use eframe::egui;

pub fn with_layout<F>(
    ui_state: &mut OpenRouterUi,
    ctx: &egui::Context,
    content: F,
) -> OpenRouterAction
where
    F: FnOnce(&mut egui::Ui, &mut OpenRouterUi) -> OpenRouterAction,
{
    let screen_name = ui_state.screen.name();
    let selected_model = ui_state.selected_model.clone();
    let streaming = ui_state.streaming;

    egui::TopBottomPanel::top("header").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("OpenRouter");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(screen_name);
            });
        });
        ui.separator();
    });

    egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.separator();
        ui.horizontal(|ui| {
            if let Some(model) = selected_model {
                ui.label(format!("Model: {}", model));
            }
            
            if streaming {
                ui.spinner();
                ui.label("Streaming...");
            }
        });
    });

    let mut action = OpenRouterAction::None;
    egui::CentralPanel::default().show(ctx, |ui| {
        action = content(ui, ui_state);
    });

    action
}


