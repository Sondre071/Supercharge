use std::process;

mod blobstorage;
mod openrouter;
mod shared;
mod scripts;

fn main() {
    jump_up_one_row();

    loop {
        let result = shared::menu::run(
            "Supercharge",
            None,
            vec!["OpenRouter", "Blobstorage", "Scripts", "Exit"],
        )
        .unwrap();

        match result {
            "OpenRouter" => openrouter::main(),
            "Scripts" => scripts::main(),
            "Blobstorage" => blobstorage::main(),
            _ => process::exit(0),
        }
    }
}

fn jump_up_one_row() {
    shared::terminal::move_cursor_pos(None, Some(-1));
    print!(
        "{clear_line}\r",
        clear_line = shared::terminal::ACTIONS.ClearLine
    );
}
