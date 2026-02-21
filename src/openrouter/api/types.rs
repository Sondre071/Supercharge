use serde::{Deserialize, Serialize};

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
pub struct MessageRequestBody<'a> {
    pub model: &'a str,
    pub input: Vec<&'a InputMessage>,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct MessageResponseStreamEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub delta: String,
}
