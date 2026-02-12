use super::state::Screen;
use super::{OpenRouterAction, OpenRouterUi};
use eframe::egui;

pub fn draw(ui_state: &mut OpenRouterUi, ctx: &egui::Context) -> OpenRouterAction {
    let action = OpenRouterAction::None;

    // Drain stream (non-blocking)
    if let Some(rx) = &ui_state.stream_rx {
        while let Ok(chunk) = rx.try_recv() {
            if let Some(last) = ui_state.chat_messages.last_mut() {
                last.content.push_str(&chunk);
            }
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("OpenRouter");
        ui.add_space(20.0);

        // Chat history
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

        // Input row
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut ui_state.chat_input);

            let can_send =
                !ui_state.streaming && !ui_state.chat_input.trim().is_empty();

            if ui.add_enabled(can_send, egui::Button::new("Send")).clicked() {
                // send() should already:
                // 1. push user message
                // 2. push empty assistant message
                // 3. spawn stream thread
                super::new_chat::send(ui_state);
            }
        });

        ui.add_space(10.0);

        if ui.button("Back").clicked() && !ui_state.streaming {
            ui_state.go_to(Screen::Main);
        }
    });

    action
}
