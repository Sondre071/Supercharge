use serde::{Deserialize, Serialize};
use std::io::{self, Write};

use crate::binary;
use crate::data;
use crate::menu;

pub fn run() {
    let data = data::get_app_data();

    let mut message_history: Vec<Message> = vec![];

    menu::write_headers("New chat", Some(&vec![&data.model, ""]));

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(Message {
            role: "user".to_string(),
            content: message.clone(),
        });

        let res = send_message(&message_history);
        println!();

        message_history.push(Message {
            role: "assistant".to_string(),
            content: res.clone(),
        });
    }
}

fn send_message(messages: &Vec<Message>) -> String {
    let data = data::get_app_data();

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

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}
