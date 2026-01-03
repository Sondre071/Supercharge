use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug)]
pub struct PromptFile {
    pub name: String,
    pub path: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct OpenRouterData {
    pub api_key: String,
    pub model: String,
    pub models: Vec<String>,
    pub parameters: Parameters,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Parameters {
    temperature: f64,
    top_p: f64,
    top_k: usize,
    frequency_penalty: f64,
    presence_penalty: f64,
    repetition_penalty: f64,
    min_p: f64,
    top_a: f64,
}
