use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PromptFile {
    pub name: String,
    pub path: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct BlobStorageData {
    pub storage_accounts: Vec<StorageAccount>,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct StorageAccount {
    pub name: String,
    pub local_files_path: String,
    pub connection_string: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct OpenRouterData {
    pub api_key: String,
    pub model: String,
    pub models: Vec<String>,
    pub parameters: Parameters,
}

#[allow(dead_code)]
#[derive(Deserialize)]
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
