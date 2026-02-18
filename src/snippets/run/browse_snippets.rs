use crate::shared::{
    menu::{self, Cursor, Item, NONE},
    statics::snippets_path,
};
use crate::snippets::utils::get_snippet_names;

use std::process;

pub fn browse_snippets() {
    let names = get_snippet_names();

    let (snippet, action) = {
        let mut cursor = Cursor::new_with_subitems(
            "Select snippet",
            NONE,
            names
                .into_iter()
                .map(|f| Item::new_with_subitems(f, vec!["Open", "Copy", "Delete"]))
                .collect(),
        );

        let Some(result) = menu::run(&mut cursor) else {
            return;
        };

        result
    };

    let mut snippet_path = snippets_path();
    snippet_path.push(snippet);

    match action.as_deref() {
        Some("Open") => {
            let _ = process::Command::new("nvim")
                .args([&snippet_path])
                .spawn()
                .expect("Failed to open neovim.")
                .wait();
        }
        Some("Copy") => {
            println!("Not implemented yet.");
        }
        Some("Delete") => {
            if let Some((choice, _)) = menu::run(&mut Cursor::new(
                "Are you sure you with to delete?",
                NONE,
                vec!["Yes", "No"],
            )) && choice == "No"
            {
                return;
            }
            
            unreachable!()
        }
        _ => {}
    }
}
