use std::process;

mod blobstorage;
mod openrouter;
mod statics;
mod utils;

fn main() {
    loop {
        let result = utils::menu::run(
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
