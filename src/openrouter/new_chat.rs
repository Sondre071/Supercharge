use crate::api;
use crate::data;
use crate::menu;

use super::types::*;
use api::openrouter::InputMessage;
use data::openrouter::get_openrouter_data;

pub fn run() {
    let data = get_openrouter_data();

    let mut message_history: Vec<InputMessage> = vec![];

    let prompt = select_prompt();

    menu::write_headers("New chat", Some(&vec![&data.model, ""]));

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(InputMessage {
            role: "user".to_string(),
            content: message.clone(),
        });

        let mut all_messages: Vec<InputMessage> = message_history.clone();

        set_sys_prompts(&prompt, &mut all_messages);

        let response_message = api::openrouter::stream_chat(&all_messages);
        println!("\n");

        if let Ok(text) = response_message {
            message_history.push(InputMessage {
                role: "assistant".to_string(),
                content: text,
            });
        }
    }
}

fn set_sys_prompts(prompt: &Option<Prompt>, messages: &mut Vec<InputMessage>) {
    if let Some(p) = prompt {
        if !p.base.is_empty() {
            let m = InputMessage {
                role: "system".to_string(),
                content: p.base.clone()
            };

            let index = messages.len().saturating_sub(8);

            messages.insert(index, m)
        }

        if !p.r#static.is_empty() {
            let m = InputMessage {
                role: "system".to_string(),
                content: p.r#static.clone()
            };

            let index = messages.len().saturating_sub(2);

            messages.insert(index, m)
        }
    } 
}

fn select_prompt() -> Option<Prompt> {
    let prompts = data::openrouter::get_prompts();

    if prompts.len() < 1 {
        return None;
    } else {
        let mut prompt_names = vec!["None"];
        prompt_names.extend(prompts.iter().map(|p| p.name.as_str()));

        let choice = menu::r#loop::run("Select prompt", None, prompt_names).unwrap();

        if choice != "None" {
            let file = prompts.iter().find(|f| f.name == choice).expect("Failed to find prompt.");
            let prompt = std::fs::read_to_string(&file.path).expect("Failed to parse prompt.");

            let mut above = Vec::new();
            let mut below = Vec::new();

            let mut in_below = false;

            for line in prompt.lines() {
                if !in_below && line.trim_start().starts_with("-----") {

                    in_below = true;
                    continue;
                }

                if in_below {
                    below.push(line);
                } else {
                    above.push(line);
                }
            }

            let above_text = above.join("\n");
            let below_text = below.join("\n");

            return Some(Prompt {
                base: above_text,
                r#static: below_text,
            });
        }
    }
    None
}
