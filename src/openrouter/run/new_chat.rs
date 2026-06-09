use crate::{
    openrouter::{
        api::{self, types::InputMessage},
        utils::settings,
    },
    shared::menu,
};

mod select_prompt;
use select_prompt::*;

pub fn new_chat() {
    let settings = settings();

    let system_prompt = select_prompt();

    let mut message_history: Vec<InputMessage> = vec![];

    let subheader = settings
        .prompt
        .as_ref()
        .map_or(vec!["".to_string()], |p| vec![p.clone(), "".to_string()]);

    menu::write_headers("New chat", subheader);

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(InputMessage {
            role: "user".to_owned(),
            content: message.clone(),
        });

        let request_messages = prepare_request_messages(&system_prompt, &message_history);
        let response_message = api::stream_chat(request_messages);

        println!("\n");

        if let Ok(text) = response_message {
            message_history.push(InputMessage {
                role: "assistant".to_owned(),
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
