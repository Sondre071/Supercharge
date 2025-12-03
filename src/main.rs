use std::process::exit;

mod data;
mod menu;
mod openrouter;
mod settings;

fn run() {
    if let Some(result) =
        menu::r#loop::run("Supercharge", None, vec!["OpenRouter", "Settings", "Exit"])
    {
        match result {
            "OpenRouter" => openrouter::run(),
            "Settings" => settings::run(),
            _ => exit(0),
        }
    }
}

fn main() {
    run()
}
