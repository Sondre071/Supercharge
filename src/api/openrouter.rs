use crate::{data};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct FetchModelsResponse {
    pub data: Vec<ModelInfo>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ModelInfo {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct MessageRequestBody<'a> {
    model: String,
    input: &'a Vec<InputMessage>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct MessageResponseStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(default)]
    delta: String,
}

pub fn fetch_models(api_key: &str) -> Vec<String> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://openrouter.ai/api/v1/models")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .expect("Failed to execute http call.");

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to read response body"));

        panic!("Non-successful http call: {}, {}", status, body);
    }

    let models_resp: FetchModelsResponse =
        response.json().expect("Failed to deserialize JSON body.");

    let model_ids: Vec<String> = models_resp.data.into_iter().map(|model| model.id).collect();

    model_ids
}

pub fn stream_chat(messages: &Vec<InputMessage>) -> Result<String, String> {
    let data = data::get_openrouter_data();

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

    let reader = BufReader::new(response);

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
                    std::io::stdout().flush().unwrap();

                    total_response.push_str(event.delta.as_str());
                }
            }
        }
    }

    Ok(total_response)
}
