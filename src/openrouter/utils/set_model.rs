use crate::openrouter;
use crate::statics;

use openrouter::utils;

use std::fs;

pub fn set_model(model: &str) {
    let mut data = utils::get_local_data();

    data.model = model.to_owned();

    let save_path = statics::openrouter_settings_path();
    let string_data = serde_json::to_string_pretty(&data).expect("Unable to serialize json.");

    fs::write(save_path, string_data).expect("Failed to save file.");
}
