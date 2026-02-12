use crate::openrouter;

use openrouter::utils;

mod main;
mod new_chat;
mod settings;
mod state;

use eframe::egui;

use state::Screen;

pub struct OpenRouterUi {
    screen: Screen,
    pub models: Vec<String>,
    pub selected_model: Option<String>,
    pub loading_models: bool,
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

    pub fn draw(&mut self, ctx: &egui::Context) -> OpenRouterAction {
        match self.screen {
            Screen::Main => main::draw(self, ctx),
            Screen::Settings => settings::draw(self, ctx),
            Screen::NewChat => new_chat::draw(self, ctx),
        }
    }

    pub fn go_to(&mut self, screen: Screen) {
        self.screen = screen;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum OpenRouterAction {
    None,
    GoBack,
}
