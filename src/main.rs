mod blobstorage;
mod openrouter;
mod scripts;
mod shared;
mod snippets;

use shared::{
    menu::{Cursor, Item, NONE},
    terminal::enable_sigint,
};

fn main() {
    init();

    let mut menu = Cursor::new_with_subitems(
        "Supercharge",
        NONE,
        vec![
            Item::new_with_subitems(
                "OpenRouter",
                vec!["New chat", "Select prompt", "Select model"],
            ),
            Item::new_with_subitems("Blobstorage", vec!["Sync", "Browse"]),
            Item::new("Scripts"),
            Item::new_with_subitems("Snippets", vec!["Browse", "Open folder"]),
            Item::new("Exit"),
        ],
    );

    loop {
        let Some((module, option)) = shared::menu::run(&mut menu) else {
            return;
        };

        match (module.as_str(), option.as_deref()) {
            ("OpenRouter", Some("New chat")) => openrouter::new_chat(),
            ("OpenRouter", Some("Select prompt")) => openrouter::select_prompt(),
            ("OpenRouter", Some("Select model")) => openrouter::select_model(),

            ("Blobstorage", Some("Sync")) => blobstorage::sync_containers(),
            ("Blobstorage", Some("Browse")) => blobstorage::browse_containers(),
            ("Scripts", None) => scripts::run_script(),

            ("Snippets", Some("Browse")) => snippets::browse_snippets(),
            ("Snippets", Some("Open folder")) => snippets::open_folder(),

            _ => return,
        }
    }
}

fn init() {
    enable_sigint();

    shared::terminal::move_cursor_pos(None, Some(-1));
    print!(
        "{clear_line}\r",
        clear_line = shared::terminal::ACTIONS.ClearLine
    );

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let arg = args[1].to_owned();

        match arg.as_str() {
            "--chat" | "-C" | "--c" => openrouter::new_chat(),
            "--blob" | "-B" | "--b" => blobstorage::sync_containers(),
            "--scripts" | "-S" | "--s" => scripts::run_script(),
            _ => {}
        }
    }
}
