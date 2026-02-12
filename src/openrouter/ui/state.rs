use crate::openrouter::{api::types::InputMessage, utils};
use eframe::egui;
use std::sync::mpsc::Receiver;

use super::{chat, menu, settings};

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Chat,
    Settings,
}

impl Screen {
    pub fn name(&self) -> &'static str {
        match self {
            Screen::Menu => "Menu",
            Screen::Chat => "Chat",
            Screen::Settings => "Settings",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum OpenRouterAction {
    None,
    GoBack,
}

pub struct OpenRouterUi {
    pub screen: Screen,

    pub chat_input: String,
    pub chat_messages: Vec<InputMessage>,
    pub stream_rx: Option<Receiver<String>>,
    pub streaming: bool,

    pub models: Vec<String>,
    pub selected_model: Option<String>,
    pub loading_models: bool,
}

impl OpenRouterUi {
    pub fn new() -> Self {
        let data = utils::get_local_data();

        Self {
            screen: Screen::Menu,

            chat_input: String::new(),
            chat_messages: Vec::new(),
            stream_rx: None,
            streaming: false,

            models: Vec::new(),
            selected_model: Some(data.model),
            loading_models: false,
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context) -> OpenRouterAction {
        match self.screen {
            Screen::Menu => menu::draw(self, ctx),
            Screen::Chat => chat::draw(self, ctx),
            Screen::Settings => settings::draw(self, ctx),
        }
    }

    pub fn go_to(&mut self, screen: Screen) {
        self.screen = screen;
    }
}
