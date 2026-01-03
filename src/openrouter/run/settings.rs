use crate::openrouter;
use crate::menu;

use openrouter::utils;
use openrouter::api;
use openrouter::run;

pub fn settings() {
    if let Some(result) =
        menu::r#loop::run("OpenRouter settings", None, vec!["Select model", "Back"])
    {
        match result {
            "Select model" => select_model(),
            _ => run::run::run(),
        }
    }
}

fn select_model() {
    let data = utils::get_local_data();

    let models = api::fetch_models(&data.api_key);

    let selected = menu::r#loop::run(
        "Select model",
        Some(vec![]),
        models.iter().map(|m| m.as_str()).collect(),
    )
    .unwrap();

    println!("{}", selected);
}
