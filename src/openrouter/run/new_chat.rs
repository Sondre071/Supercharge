use crate::shared::menu;
use crate::{
    openrouter::{
        api::{self, types::InputMessage},
        utils::settings,
    },
    shared::statics,
};

pub fn new_chat() {
    let settings = settings();

    let system_prompt = get_system_prompt();

    let mut message_history: Vec<InputMessage> = vec![];

    menu::write_headers("New chat", vec![&settings.model, ""]);

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(InputMessage {
            role: "user".to_string(),
            content: message.clone(),
        });

        let request_messages = prepare_request_messages(&system_prompt, &message_history);
        let response_message = api::stream_chat(request_messages);

        println!("\n");

        if let Ok(text) = response_message {
            message_history.push(InputMessage {
                role: "assistant".to_string(),
                content: text,
            });
        }
    }
}

fn prepare_request_messages<'a>(
    system_prompt: &'a Option<InputMessage>,
    message_history: &'a [InputMessage],
) -> Vec<&'a InputMessage> {
    let mut messages: Vec<&InputMessage> = message_history.iter().collect();

    if let Some(sys_p) = system_prompt {
        let insert_pos = messages.len().saturating_sub(1);
        messages.insert(insert_pos, sys_p);
    };

    messages
}

fn get_system_prompt() -> Option<InputMessage> {
    let Some(file_name) = &settings().prompt else {
        return None;
    };

    let mut file_path = statics::prompts_dir();
    file_path.push(file_name);

    let content = std::fs::read_to_string(file_path).expect("Failed to read prompt content.");

    Some(InputMessage {
        role: "user".to_string(),
        content,
    })
}
