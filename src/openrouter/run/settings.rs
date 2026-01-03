use crate::menu;
use crate::openrouter;

use openrouter::run;

pub fn settings() {
    if let Some(result) = menu::run("OpenRouter settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => run::select_model(),
            _ => run::run(),
        }
    }
}
