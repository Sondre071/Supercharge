mod blobstorage;
mod openrouter;
mod scripts;
mod shared;
mod snippets;

use shared::menu::{Cursor, Item, NONE};

fn main() {
    jump_up_one_row();

    let mut menu = Cursor::new_with_subitems(
        "Supercharge",
        NONE,
        vec![
            Item::new_with_subitems("OpenRouter", vec!["New chat", "Settings"]),
            Item::new_with_subitems("Blobstorage", vec!["Sync", "Browse"]),
            Item::new("Scripts"),
            Item::new_with_subitems("Snippets", vec!["Browse", "Add new"]),
            Item::new("Exit"),
        ],
    );

    loop {
        let Some((module, option)) = shared::menu::run(&mut menu) else {
            return;
        };

        match (module.as_str(), option.as_deref()) {
            ("OpenRouter", Some("New chat")) => openrouter::new_chat(),
            ("OpenRouter", Some("Settings")) => openrouter::settings(),

            ("Blobstorage", Some("Sync")) => blobstorage::sync_containers(),
            ("Blobstorage", Some("Browse")) => blobstorage::browse_containers(),
            ("Scripts", None) => scripts::main(),

            ("Snippets", Some("Browse")) => snippets::browse_snippets(),
            ("Snippets", Some("Add new")) => snippets::add_snippet(),

            _ => return,
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
