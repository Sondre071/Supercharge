use crate::openrouter;

use super::state::Screen;
use super::{OpenRouterAction, OpenRouterUi};
use eframe::egui;
use openrouter::api;
use openrouter::utils;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    let mut action = OpenRouterAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("OpenRouter Settings");
        ui.add_space(20.0);

        ui.group(|ui| {
            ui.label("Model Selection");
            ui.add_space(10.0);

            if ui.button("Load Models").clicked() {
                load_models(ui_state);
            }

            if !ui_state.models.is_empty() {
                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        for model in &ui_state.models {
                            let is_selected = ui_state.selected_model.as_ref() == Some(model);

                            if ui.selectable_label(is_selected, model).clicked() {
                                ui_state.selected_model = Some(model.clone());
                                utils::set_model(model);
                            }
                        }
                    });
            }

            if let Some(model) = &ui_state.selected_model {
                ui.add_space(10.0);
                ui.label(format!("Selected: {}", model));
            }
        });

        ui.add_space(20.0);

        if ui.button("Back").clicked() {
            ui_state.go_to(Screen::Main);
        }

        if ui.button("Exit OpenRouter").clicked() {
            action = OpenRouterAction::GoBack;
        }
    });

    action
}

fn load_models(ui_state: &mut OpenRouterUi) {
    ui_state.loading_models = true;

    let data = utils::get_local_data();
    ui_state.models = api::fetch_models(&data.api_key);

    ui_state.loading_models = false;
}
