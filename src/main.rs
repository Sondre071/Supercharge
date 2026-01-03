use std::process::exit;

mod menu;
mod openrouter;
mod blobstorage;
mod statics;
mod terminal;
mod utils;

fn main() {
    if let Some(result) =
        menu::r#loop::run("Supercharge", None, vec!["OpenRouter", "Blobstorage", "Exit"])
    {
        match result {
            "OpenRouter" => openrouter::run::run(),
            "Blobstorage" => blobstorage::run::run(),
            _ => exit(0),
        }
    }
}
