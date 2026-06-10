use crate::shared::menu;
use crate::{
    openrouter::{
        api::{self, types::InputMessage},
        utils::get_prompts,
    },
    shared::{menu::Cursor, menu::NONE, statics},
};
use std::fs;
use std::io::Write;
use std::iter::once;
use std::path::PathBuf;

pub fn new_chat() {
    let prompt = {
        let cursor = &mut Cursor::new(
            "Select prompt",
            NONE,
            once("None")
                .chain(get_prompts().iter().map(|p| p.name.as_str()))
                .collect(),
            None,
        );
        menu::run(cursor)
    };

    // User wants to exit the menu.
    if prompt.is_none() {
        return;
    };

    let system_prompt = 'block: {
        let (prompt_name, _) = prompt.unwrap();

        if prompt_name == "None" {
            break 'block None;
        };

        let mut file_path = statics::prompts_dir();
        file_path.push(prompt_name);

        let content = fs::read_to_string(file_path).expect("Failed to read prompt file content.");

        Some(InputMessage {
            role: "system",
            content,
        })
    };

    let mut message_history: Vec<InputMessage> = vec![];

    menu::write_headers("New chat", vec![""]);

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(InputMessage {
            role: "user",
            content: message,
        });

        let message_history_with_system_prompt = {
            let mut messages: Vec<&InputMessage> = message_history.iter().collect();

            if let Some(system_prompt) = &system_prompt {
                messages.insert(0, system_prompt);
            }

            messages
        };

        let response_message = api::stream_chat(message_history_with_system_prompt)
            .expect("Failed to received response message.");

        println!("\n");

        message_history.push(InputMessage {
            role: "assistant",
            content: response_message,
        });
        
        // wip
        //write_history_to_file(&message_history);
    }
}

#[allow(dead_code)]
fn write_history_to_file(messages: &Vec<InputMessage>) {
    let mut path = PathBuf::new();
    path.push(std::env::current_dir().unwrap());
    path.push("history_output.txt");

    let file = std::fs::File::create(path).expect("Failed to open output file.");
    let mut writer = std::io::BufWriter::new(file);

    for line in messages {
        writeln!(writer, "{}: {}", line.role, line.content)
            .expect("Failed to write line to output file.");
    }
    
    writer.flush().expect("Failed to flush bufwriter.");
}
