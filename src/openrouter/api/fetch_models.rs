use crate::openrouter;

use openrouter::api::types::FetchModelsResponse;

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
