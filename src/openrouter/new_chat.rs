use futures_util::StreamExt;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

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
            role: "user",
            content: message.clone(),
        });

        let res = send_message(&message_history);
        println!();

        message_history.push(Message {
            role: "assistant",
            content: res.clone(),
        });
    }
}

#[tokio::main]
async fn send_message(messages: &Vec<Message>) -> String {
    let data = data::get_app_data();

    let body = RequestBody {
        model: &data.model,
        input: messages.clone(),
        stream: true,
    };

    let res = reqwest::Client::new()
        .post("https://openrouter.ai/api/v1/responses")
        .json(&body)
        .bearer_auth(&data.api_key)
        .send()
        .await
        .expect("Request failed.")
        .error_for_status()
        .expect("Non success status from API.");

    let mut stream = res.bytes_stream();
    let mut full_response = String::new();

    print!("\x1b[0;96m");

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);

                for line in text.lines() {
                    if line.is_empty() || line == ": OPENROUTER PROCESSING" {
                        continue;
                    }

                    if let Some(json_str) = line.strip_prefix("data: ") {
                        if json_str == "[DONE]" {
                            continue;
                        }

                        if let Ok(event) = serde_json::from_str::<StreamEvent>(json_str) {
                            if event.r#type == "response.output_text.delta" {
                                print!("{}", event.delta);
                                io::stdout().flush().unwrap();

                                full_response.push_str(&event.delta);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e);
                break;
            }
        }
    }

    print!("\x1b[0m\n");

    full_response
}

#[derive(Serialize, Clone)]
struct Message<'a> {
    role: &'a str,
    content: String,
}

#[derive(Serialize)]
struct RequestBody<'a> {
    model: &'a str,
    input: Vec<Message<'a>>,
    stream: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct StreamEvent {
    r#type: String,

    #[serde(default)]
    logprobs: Vec<serde_json::Value>,

    #[serde(default)]
    output_index: u32,

    #[serde(default)]
    item_id: String,

    #[serde(default)]
    content_index: u32,

    #[serde(default)]
    delta: String,

    #[serde(default)]
    sequence_number: u32,
}
