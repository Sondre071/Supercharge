use crate::openrouter::api::{self, types::InputMessage};
use super::state::Screen;
use super::{OpenRouterAction, OpenRouterUi};

use eframe::egui;
use std::sync::mpsc;
use std::thread;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    
    // Drain stream (non-blocking) and append into last assistant message
    if let Some(rx) = &ui_state.stream_rx {
        while let Ok(delta) = rx.try_recv() {
            if let Some(last) = ui_state.chat_messages.last_mut() {
                // last is InputMessage, append to its content
                last.content.push_str(&delta);
            }
        }

        // stream ended when sender drops
        match rx.try_recv() {
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                ui_state.streaming = false;
                ui_state.stream_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Ok(delta) => {
                if let Some(last) = ui_state.chat_messages.last_mut() {
                    last.content.push_str(&delta);
                }
            }
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("New Chat");
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for msg in &ui_state.chat_messages {
                ui.group(|ui| {
                    ui.strong(&msg.role);
                    ui.label(&msg.content);
                });
                ui.add_space(6.0);
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut ui_state.chat_input);

            let can_send = !ui_state.streaming && !ui_state.chat_input.trim().is_empty();
            if ui.add_enabled(can_send, egui::Button::new("Send")).clicked() {
                send(ui_state);
            }
        });

        ui.add_space(10.0);

        if ui.button("Back").clicked() && !ui_state.streaming {
            ui_state.go_to(Screen::Main);
        }
    });

    OpenRouterAction::None
}

pub fn send(ui_state: &mut OpenRouterUi) {
    let prompt = ui_state.chat_input.trim().to_string();
    ui_state.chat_input.clear();

    // push user message
    ui_state.chat_messages.push(InputMessage {
        role: "user".to_string(),
        content: prompt,
    });

    // push empty assistant message that we stream into
    ui_state.chat_messages.push(InputMessage {
        role: "assistant".to_string(),
        content: String::new(),
    });

    let messages_for_api = ui_state.chat_messages.clone();

    let (tx, rx) = mpsc::channel();
    ui_state.stream_rx = Some(rx);
    ui_state.streaming = true;

    thread::spawn(move || {
        let _ = api::stream_chat(messages_for_api, tx);
        // tx drops at end -> UI sees Disconnected
    });
}
