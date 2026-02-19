use crate::openrouter::{
    api::{self, types::InputMessage},
    utils,
};
use crate::shared::menu::{self, Cursor, NONE};

pub fn new_chat() {
    let data = utils::get_local_data();

    let mut message_history: Vec<InputMessage> = vec![];

    let prompt = {
        let Some(prompt) = select_prompt() else {
            return;
        };

        prompt
    };

    menu::write_headers("New chat", vec![&data.model, ""]);

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(InputMessage {
            role: "user".to_string(),
            content: message.clone(),
        });

        let mut all_messages: Vec<InputMessage> = message_history.clone();

        set_sys_prompts(&prompt, &mut all_messages);

        let response_message = api::stream_chat(&all_messages);
        println!("\n");

        if let Ok(text) = response_message {
            message_history.push(InputMessage {
                role: "assistant".to_string(),
                content: text,
            });
        }
    }
}

fn set_sys_prompts(prompt: &Option<String>, messages: &mut Vec<InputMessage>) {
    let Some(p) = prompt else { return };

    let m = InputMessage {
        role: "system".to_string(),
        content: p.clone(),
    };

    let index = messages.len().saturating_sub(2);

    messages.insert(index, m)
}

fn select_prompt() -> Option<Option<String>> {
    let prompts = utils::get_prompts();

    if prompts.is_empty() {
        Some(None)
    } else {
        let mut prompt_names = vec!["None"];
        prompt_names.extend(prompts.iter().map(|p| p.name.as_str()));

        if let Some((choice, _)) = menu::run(&mut Cursor::new("Select prompt", NONE, prompt_names))
        {
            if choice == "None" {
                Some(None)
            } else {
                let file = prompts
                    .iter()
                    .find(|f| f.name == choice)
                    .expect("Failed to find prompt.");
                let prompt = std::fs::read_to_string(&file.path).expect("Failed to parse prompt.");

                Some(Some(prompt))
            }
        } else {
            None
        }
    }
}
