use serde::Deserialize;
use serde_json;

use crate::data;
use crate::menu;

pub fn run() {
    if let Some(result) = menu::r#loop::run("Settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => select_model(),
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    id: String,
}

#[tokio::main]
async fn select_model() {
    let data = data::get_app_data();

    let client = reqwest::Client::new()
        .get("https://openrouter.ai/api/v1/models")
        .bearer_auth(&data.api_key);

    let res = client.send().await.expect("Request failed.");

    let models: ModelsResponse = res.json().await.expect("Failed to read response body.");

    let mods: Vec<&str> = models.data.iter().map(|model| model.id.as_str()).collect();

    if let Some(result) = menu::r#loop::run("Select model", None, mods) {
        match result {
            _ => {}
        }
    }
}
