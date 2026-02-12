use super::state::{OpenRouterAction, OpenRouterUi, Screen};
use crate::openrouter::{api, utils};
use eframe::egui;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    let mut action = OpenRouterAction::None;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");
        ui.add_space(20.0);

        show_model_section(ui, ui_state);

        ui.add_space(20.0);

        if ui.button("Back to Menu").clicked() {
            ui_state.go_to(Screen::Menu);
        }

        ui.add_space(10.0);

        if ui.button("Exit OpenRouter").clicked() {
            action = OpenRouterAction::GoBack;
        }
    });

    action
}

fn show_model_section(ui: &mut egui::Ui, ui_state: &mut OpenRouterUi) {
    ui.group(|ui| {
        ui.label("Model Selection");
        ui.add_space(10.0);

        if ui.button("Load Models").clicked() {
            load_models(ui_state);
        }

        if !ui_state.models.is_empty() {
            ui.add_space(10.0);
            show_model_list(ui, ui_state);
        }

        if let Some(model) = &ui_state.selected_model {
            ui.add_space(10.0);
            ui.label(format!("Selected: {}", model));
        }
    });
}

fn show_model_list(ui: &mut egui::Ui, ui_state: &mut OpenRouterUi) {
    let models = ui_state.models.clone();

    egui::ScrollArea::vertical()
        .max_height(400.0)
        .show(ui, |ui| {
            for model in &models {
                let is_selected = ui_state.selected_model.as_ref() == Some(model);

                if ui.selectable_label(is_selected, model).clicked() {
                    select_model(ui_state, model);
                }
            }
        });
}

fn load_models(ui_state: &mut OpenRouterUi) {
    ui_state.loading_models = true;

    let data = utils::get_local_data();
    ui_state.models = api::fetch_models(&data.api_key);

    ui_state.loading_models = false;
}

fn select_model(ui_state: &mut OpenRouterUi, model: &str) {
    ui_state.selected_model = Some(model.to_string());
    utils::set_model(model);
}
