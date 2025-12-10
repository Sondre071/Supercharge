use std::process::exit;

mod data;
mod menu;
mod openrouter;
mod settings;
mod blobstorage;
mod statics;
mod api;
mod download;

fn main() {
    if let Some(result) =
        menu::r#loop::run("Supercharge", None, vec!["OpenRouter", "Download", "Blobstorage", "Settings", "Exit"])
    {
        match result {
            "OpenRouter" => openrouter::run(),
            "Download" => download::run(),
            "Blobstorage" => blobstorage::run(),
            "Settings" => settings::run(),
            _ => exit(0),
        }
    }
}
