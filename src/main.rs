use std::process::exit;

mod blobstorage;
mod menu;
mod openrouter;
mod statics;
mod terminal;
mod utils;

fn main() {
    loop {
        let result = menu::run(
            "Supercharge",
            None,
            vec!["OpenRouter", "Blobstorage", "Exit"],
        )
        .unwrap();

        match result {
            "OpenRouter" => openrouter::run::run(),
            "Blobstorage" => blobstorage::run::run(),
            _ => exit(0),
        }
    }
}
