use crate::shared::statics;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct PromptFile {
    pub name: String,
    pub path: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub api_key: String,
    pub model: String,
    pub models: Vec<String>,
    pub parameters: Parameters,
    pub prompt: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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

impl Settings {
    pub fn load_from_disk() -> Self {
        let file = File::open(statics::openrouter_settings_path())
            .expect("Failed to open OpenRouter settings.");

        let reader = BufReader::new(file);

        serde_json::from_reader(reader).expect("Failed to deserialize OpenRouter settings.")
    }

    pub fn save_to_disk(&self) {
        let save_path = statics::openrouter_settings_path();

        save_json_atomic(save_path, &self);
    }
}

fn save_json_atomic<T: serde::Serialize>(path: PathBuf, value: &T) {
    let temp_path = path.with_extension("json.tmp");

    let data = serde_json::to_string_pretty(value).expect("Failed to serialize value");

    fs::write(&temp_path, data).expect("Failed to write serialized data to file.");

    fs::remove_file(&path).expect("Failed to remove file before replacement.");

    fs::rename(&temp_path, path).expect("Failed to replace old file with new.");
}
