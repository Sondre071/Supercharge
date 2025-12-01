use std::process::exit;

mod menu;
mod data;
mod openrouter;

fn run() {
    let options = vec!["OpenRouter", "Quit"];

    if let Some(result) = menu::r#loop::run("Supercharge", None, options) {
        match result {
            "OpenRouter" => openrouter::run(),
            _ => exit(0),
        }
    }
}

fn main() {
    run()
}
