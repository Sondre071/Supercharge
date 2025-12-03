use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader};
use std::path::Path;

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

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Data {
    pub api_key: String,
    pub model: String,
    pub models: Vec<String>,
    pub parameters: Parameters,
    pub prompts: String,
}

pub fn get_app_data() -> Result<Data, io::Error> {
    let home_dir = env::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No home directory found."))?;

    let relative_path = Path::new(".supercharge").join("supercharge.json");

    let full_path = home_dir.join(relative_path);

    let file = File::open(full_path)?;
    let reader = BufReader::new(file);

    let data: Data = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(data)
}
