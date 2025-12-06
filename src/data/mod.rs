use std::env;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub struct Data {
    pub api_key: String,
    pub model: String,
    pub models: Vec<String>,
    pub parameters: Parameters,
    pub prompts: String,
}

pub fn get_app_data() -> Data {

    let mut path = env::home_dir().unwrap();
    path.push(".supercharge");
    path.push("data");
    path.push("supercharge.json");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: Data = serde_json::from_reader(reader).unwrap();

    data
}
