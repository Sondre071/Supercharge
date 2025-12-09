use crate::api;
use crate::data;
use crate::menu;

use data::openrouter::get_openrouter_data;
use api::openrouter::fetch_models;

pub fn run() {
    if let Some(result) = menu::r#loop::run("Settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => select_model(),
            _ => {}
        }
    }
}

fn select_model() {
    let data = get_openrouter_data();

    let models = fetch_models(&data.api_key);

    let selected = menu::r#loop::run(
        "Select model",
        Some(vec![]),
        models.iter().map(|m| m.as_str()).collect(),
    ).unwrap();

    println!("{}", selected);
}
