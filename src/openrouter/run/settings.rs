use crate::menu;
use crate::openrouter;

use openrouter::api;
use openrouter::run;
use openrouter::utils;

pub fn settings() {
    if let Some(result) = menu::run("OpenRouter settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => select_model(),
            _ => run::run::run(),
        }
    }
}

fn select_model() {
    let data = utils::get_local_data();

    let models = api::fetch_models(&data.api_key);

    let selected = menu::run(
        "Select model",
        Some(vec![]),
        models.iter().map(|m| m.as_str()).collect(),
    )
    .unwrap();

    println!("{}", selected);
}
