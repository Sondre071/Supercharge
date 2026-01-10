use crate::openrouter;
use crate::statics;

use openrouter::utils::types::OpenRouterData;
use std::fs::File;
use std::io::BufReader;

pub fn get_local_data() -> OpenRouterData {
    let file = File::open(statics::openrouter_settings_path())
        .expect("Failed to open OpenRouter settings.");

    let reader = BufReader::new(file);
    let data: OpenRouterData =
        serde_json::from_reader(reader).expect("Failed to deserialize openrouter settings.");

    data
}
