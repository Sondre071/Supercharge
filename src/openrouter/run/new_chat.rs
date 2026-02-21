use crate::openrouter::{
    api::{self, types::InputMessage},
    utils,
};
use crate::shared::menu::{self, Cursor, NONE};

pub fn new_chat() {
    let data = utils::get_local_data();

    let mut message_history: Vec<InputMessage> = vec![];

    let system_prompt = match select_prompt() {
        Some(Some(prompt)) => Some(InputMessage {
            role: "system".to_string(),
            content: prompt,
        }),
        Some(None) => None,
        None => return,
    };

    menu::write_headers("New chat", vec![&data.model, ""]);

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
    let mut result: Vec<&InputMessage> = message_history.iter().collect();

    if let Some(sys) = system_prompt {
        let insert_pos = result.len().saturating_sub(1);
        result.insert(insert_pos, sys)
    }

    result
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
