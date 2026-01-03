use crate::openrouter;

use openrouter::utils;
use openrouter::api::types::{InputMessage, MessageRequestBody, MessageResponseStreamEvent};

use std::io;
use std::io::{BufRead, Write};

pub fn stream_chat(messages: &Vec<InputMessage>) -> Result<String, String> {
    let data = utils::get_local_data();

    let body = MessageRequestBody {
        model: data.model,
        input: messages,
        stream: true,
    };

    let body_json = serde_json::to_string(&body)
        .map_err(|e| format!("Failed to marshal request body: {}", e))?;

    let client = reqwest::blocking::Client::new();

    let response = client
        .post("https://openrouter.ai/api/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", data.api_key))
        .body(body_json)
        .send()
        .map_err(|e| format!("Failed to execute request: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to read response body"));

        return Err(format!(
            "Non-success HTTP status: {}, {}",
            status, body_text
        ));
    }

    let reader = io::BufReader::new(response);

    let mut total_response = String::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read stream: {}", e))?;
        let line = line.trim();

        if line.is_empty() || line == ": OPENROUTER PROCESSING" {
            continue;
        }

        if line.starts_with("data: ") {
            let json_str = line.strip_prefix("data: ").unwrap();

            if json_str == "[DONE]" {
                break;
            }

            if let Ok(event) = serde_json::from_str::<MessageResponseStreamEvent>(json_str) {
                if event.event_type == "response.output_text.delta" && !event.delta.is_empty() {
                    print!("\x1b[0;96m{}\x1b[0m", event.delta);
                    io::stdout().flush().unwrap();

                    total_response.push_str(event.delta.as_str());
                }
            }
        }
    }

    Ok(total_response)
}