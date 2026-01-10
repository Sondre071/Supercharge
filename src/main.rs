use std::process;

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
            "OpenRouter" => openrouter::main(),
            "Blobstorage" => blobstorage::main(),
            _ => process::exit(0),
        }
    }
}
