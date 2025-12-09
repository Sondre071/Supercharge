use super::types::PromptFile;
use crate::statics;
use super::types::*;

use std::fs::File;
use std::io::BufReader;

pub fn get_openrouter_data() -> OpenRouterData {
    let file = File::open(statics::openrouter_settings_path())
        .expect("Failed to open OpenRouter settings.");

    let reader = BufReader::new(file);
    let data: OpenRouterData =
        serde_json::from_reader(reader).expect("Failed to deserialize openrouter settings.");

    data
}

pub fn get_prompts() -> Vec<PromptFile> {
    let entries =
        std::fs::read_dir(statics::prompts_dir()).expect("Failed to read from prompts folder.");

    let prompts: Vec<PromptFile> = entries
        .filter_map(|file| {
            let file = file.expect("Failed to parse file.");
            let file_name = file.file_name().to_str().unwrap().to_string();
            let file_path = file.path();

            Some(PromptFile {
                name: file_name,
                path: file_path,
            })
        })
        .collect();

    prompts
}
