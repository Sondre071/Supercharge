use api::openrouter::InputMessage;

use crate::api;
use crate::data;
use crate::menu;
use crate::statics;

pub fn run() {
    let data = data::get_openrouter_data();

    let mut message_history: Vec<InputMessage> = {
        let prompts = get_prompts();

        if prompts.len() < 1 {
            vec![]
        } else {
            let mut prompt_names = vec!["None"];
            prompt_names.extend(prompts.iter().map(|p| p.name.as_str()));

            let choice = menu::r#loop::run("Select prompt", None, prompt_names).unwrap();

            if choice != "None" {
                let file = prompts.iter().find(|f| f.name == choice).unwrap();
                let prompt = std::fs::read_to_string(&file.path).expect("Failed to parse prompt.");

                vec![InputMessage {
                    role: "system".to_string(),
                    content: prompt,
                }]
            } else {
                vec![]
            }
        }
    };

    menu::write_headers("New chat", Some(&vec![&data.model, ""]));

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(InputMessage {
            role: "user".to_string(),
            content: message.clone(),
        });

        let res = api::openrouter::stream_chat(&message_history);
        println!("\n");

        if let Ok(text) = res {
            message_history.push(InputMessage {
                role: "assistant".to_string(),
                content: text,
            });
        }
    }
}

#[derive(Debug)]
struct PromptFile {
    name: String,
    path: std::path::PathBuf,
}

fn get_prompts() -> Vec<PromptFile> {
    let entries =
        std::fs::read_dir(statics::prompts_dir()).expect("Failed to read from prompts folder.");

    let prompts: Vec<PromptFile> = entries
        .filter_map(|file| {
            let file = file.expect("Failed to parse file.");
            let file_name = file.file_name().to_str().unwrap().to_string();
            let file_path = file.path();

            Some(PromptFile {
                name: file_name,
                path: file_path,
            })
        })
        .collect();

    prompts
}
