use std::process::exit;

mod binary;
mod data;
mod menu;
mod openrouter;
mod settings;
mod blobstorage;

fn main() {
    if let Some(result) =
        menu::r#loop::run("Supercharge", None, vec!["OpenRouter", "Blobstorage", "Settings", "Exit"])
    {
        match result {
            "OpenRouter" => openrouter::run(),
            "Blobstorage" => blobstorage::run(),
            "Settings" => settings::run(),
            _ => exit(0),
        }
    }
}
