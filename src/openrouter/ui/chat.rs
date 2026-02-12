use super::state::{OpenRouterAction, OpenRouterUi, Screen};
use crate::openrouter::api::{self, types::InputMessage};
use eframe::egui;
use std::sync::mpsc;
use std::thread;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    handle_stream(ui_state);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Chat");
        ui.add_space(12.0);

        show_messages(ui, &ui_state.chat_messages);

        ui.separator();

        show_input(ui, ui_state);

        ui.add_space(10.0);

        if ui.button("Back to Menu").clicked() && !ui_state.streaming {
            clear_chat(ui_state);
            ui_state.go_to(Screen::Menu);
        }
    });

    OpenRouterAction::None
}

fn handle_stream(ui_state: &mut OpenRouterUi) {
    let Some(rx) = &ui_state.stream_rx else {
        return;
    };

    loop {
        match rx.try_recv() {
            Ok(chunk) => append_to_last_message(&mut ui_state.chat_messages, &chunk),
            Err(mpsc::TryRecvError::Empty) => break,
            Err(mpsc::TryRecvError::Disconnected) => {
                ui_state.streaming = false;
                ui_state.stream_rx = None;
                break;
            }
        }
    }
}

fn show_messages(ui: &mut egui::Ui, messages: &[InputMessage]) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for msg in messages {
            ui.group(|ui| {
                ui.strong(&msg.role);
                ui.label(&msg.content);
            });
            ui.add_space(6.0);
        }
    });
}

fn show_input(ui: &mut egui::Ui, ui_state: &mut OpenRouterUi) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut ui_state.chat_input);

        let can_send = !ui_state.streaming && !ui_state.chat_input.trim().is_empty();

        if ui
            .add_enabled(can_send, egui::Button::new("Send"))
            .clicked()
        {
            send_message(ui_state);
        }
    });
}

fn send_message(ui_state: &mut OpenRouterUi) {
    let user_message = ui_state.chat_input.trim().to_string();
    ui_state.chat_input.clear();

    add_user_message(&mut ui_state.chat_messages, user_message);
    add_empty_assistant_message(&mut ui_state.chat_messages);

    start_streaming(ui_state);
}

fn start_streaming(ui_state: &mut OpenRouterUi) {
    let messages = ui_state.chat_messages.clone();
    let (tx, rx) = mpsc::channel();

    ui_state.stream_rx = Some(rx);
    ui_state.streaming = true;

    thread::spawn(move || {
        let _ = api::stream_chat(messages, tx);
    });
}

fn add_user_message(messages: &mut Vec<InputMessage>, content: String) {
    messages.push(InputMessage {
        role: "user".to_string(),
        content,
    });
}

fn add_empty_assistant_message(messages: &mut Vec<InputMessage>) {
    messages.push(InputMessage {
        role: "assistant".to_string(),
        content: String::new(),
    });
}

fn append_to_last_message(messages: &mut [InputMessage], chunk: &str) {
    if let Some(last) = messages.last_mut() {
        last.content.push_str(chunk);
    }
}

fn clear_chat(ui_state: &mut OpenRouterUi) {
    ui_state.chat_messages.clear();
    ui_state.chat_input.clear();
}
