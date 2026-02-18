use std::process;

//mod blobstorage;
//mod openrouter;
//mod scripts;
mod shared;
//mod snippets;

use shared::menu::{Item, Menu};

fn main() {
    jump_up_one_row();

    loop {
        let menu = Menu::new_with_subitems(
            "Supercharge",
            vec![""],
            vec![
                Item::new_with_subitems("OpenRouter", vec!["New chat", "Settings"]),
                Item::new_with_subitems("Blobstorage", vec!["Sync", "Browse", "Settings"]),
                Item::new("Scripts"),
                Item::new_with_subitems("Snippets", vec!["Run", "Create new"]),
                Item::new("Exit"),
            ],
        );
        //let result = shared::menu::run(menu.clone()).unwrap();
        let result = shared::menu::run(menu.clone()).unwrap();

        //match result {
        //    "OpenRouter" => openrouter::main(),
        //    "Blobstorage" => blobstorage::main(),
        //    "Scripts" => scripts::main(),
        //    "Snippets" => snippets::main(),
        //    _ => process::exit(0),
        //}
    }
}

fn jump_up_one_row() {
    shared::terminal::move_cursor_pos(None, Some(-1));
    print!(
        "{clear_line}\r",
        clear_line = shared::terminal::ACTIONS.ClearLine
    );
}
