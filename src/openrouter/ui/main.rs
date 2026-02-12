use crate::openrouter;

use openrouter::api;
use openrouter::utils;
use eframe::egui;


pub struct OpenRouterUi {
    screen: Screen,
    models: Vec<String>,
    selected_model: Option<String>,
    loading_models: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Main,
    Settings,
    NewChat,
}

impl OpenRouterUi {
    pub fn new() -> Self {
        let data = utils::get_local_data();

        Self {
            screen: Screen::Main,
            models: Vec::new(),
            selected_model: Some(data.model),
            loading_models: false,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) -> OpenRouterAction {
        match self.screen {
            Screen::Main => self.ui_main(ctx),
            Screen::Settings => self.ui_settings(ctx),
            Screen::NewChat => self.ui_new_chat(ctx),
        }
    }

    fn ui_main(&mut self, ctx: &egui::Context) -> OpenRouterAction {
        let mut action = OpenRouterAction::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OpenRouter");
            ui.add_space(20.0);

            if ui.button("New Chat").clicked() {
                self.screen = Screen::NewChat;
            }

            if ui.button("Settings").clicked() {
                self.screen = Screen::Settings;
            }

            if ui.button("Back").clicked() {
                action = OpenRouterAction::GoBack;
            }
        });

        action
    }

    fn ui_new_chat(&mut self, ctx: &egui::Context) -> OpenRouterAction {
        let action = OpenRouterAction::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("New Chat");
            ui.add_space(20.0);

            ui.label("Chat UI goes here.");

            if ui.button("Back").clicked() {
                self.screen = Screen::Main;
            }
        });

        action
    }

    fn ui_settings(&mut self, ctx: &egui::Context) -> OpenRouterAction {
        let mut action = OpenRouterAction::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("OpenRouter Settings");
            ui.add_space(20.0);

            ui.group(|ui| {
                ui.label("Model Selection");
                ui.add_space(10.0);

                if ui.button("Load Models").clicked() {
                    self.load_models();
                }

                if !self.models.is_empty() {
                    ui.add_space(10.0);

                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                            for model in &self.models {
                                let is_selected =
                                    self.selected_model.as_ref() == Some(model);

                                if ui.selectable_label(is_selected, model).clicked() {
                                    self.selected_model = Some(model.clone());
                                    utils::set_model(model);
                                }
                            }
                        });
                }

                if let Some(model) = &self.selected_model {
                    ui.add_space(10.0);
                    ui.label(format!("Selected: {}", model));
                }
            });

            ui.add_space(20.0);

            if ui.button("Back").clicked() {
                self.screen = Screen::Main;
            }

            if ui.button("Exit OpenRouter").clicked() {
                action = OpenRouterAction::GoBack;
            }
        });

        action
    }

    fn load_models(&mut self) {
        self.loading_models = true;

        let data = utils::get_local_data();
        self.models = api::fetch_models(&data.api_key);

        self.loading_models = false;
    }
}

pub enum OpenRouterAction {
    None,
    GoBack,
}