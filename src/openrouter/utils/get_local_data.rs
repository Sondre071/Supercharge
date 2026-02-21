use crate::openrouter;
use crate::shared::statics;

use openrouter::utils::types::Settings;
use std::fs::File;
use std::io::BufReader;

pub fn get_local_data() -> Settings {
    let file = File::open(statics::openrouter_settings_path())
        .expect("Failed to open OpenRouter settings.");

    let reader = BufReader::new(file);
    let data: Settings =
        serde_json::from_reader(reader).expect("Failed to deserialize openrouter settings.");

    data
}
