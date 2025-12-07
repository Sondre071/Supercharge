use api::openrouter::InputMessage;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::api;
use crate::binary;
use crate::data;
use crate::menu;
use crate::statics;

pub fn run() {
    let data = data::get_openrouter_data();

    let mut message_history: Vec<InputMessage> = {
        let prompts = get_prompts();

        if prompts.len() > 0 {
            let mut prompt_names = vec!["None"];
            prompt_names.extend(prompts.iter().map(|p| p.name.as_str()));

            let choice = menu::r#loop::run("Select prompt", None, prompt_names).unwrap();

            if choice != "None" {
                let file = prompts.iter().find(|f| f.name == choice).unwrap();
                let prompt = std::fs::read_to_string(&file.path).expect("Failed to parse prompt.");

                vec![InputMessage {
                    role: "system".to_string(),
                    content: prompt,
                }];
            }
        }

        vec![]
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
                content: text.clone(),
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

fn send_message(messages: &Vec<api::openrouter::InputMessage>) -> String {
    let data = data::get_openrouter_data();

    let messages_json = serde_json::to_string(&messages).expect("Failed to serialize messages");

    let args = vec![
        "--api-key".to_string(),
        data.api_key.clone(),
        "--model".to_string(),
        data.model.clone(),
        "--messages".to_string(),
        messages_json,
    ];

    let mut binary_path = std::env::current_exe().unwrap();
    binary_path.pop();
    binary_path.push("bin");
    binary_path.push("openrouter");
    binary_path.push("post_message.exe");

    let mut reader = match binary::run_streaming(binary_path.to_str().unwrap(), args) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to start binary: {}", e);
            return String::new();
        }
    };

    let mut full_response = String::new();

    print!("\x1b[0;96m");
    io::stdout().flush().unwrap();

    let mut buffer = [0u8; 1024];
    loop {
        match reader.stdout().read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let text = String::from_utf8_lossy(&buffer[0..n]);

                print!("{}", text);
                io::stdout().flush().unwrap();

                full_response.push_str(&text);
            }
            Err(e) => {
                eprintln!("Error reading stdout: {}", e);
                break;
            }
        }
    }

    print!("\x1b[0m\n");

    if let Err(e) = reader.wait() {
        eprintln!("{}", e);
    }

    full_response
}
