use crate::{
    shared::{
        menu::{self, Cursor, Item, NONE},
        statics::snippets_dir,
        terminal::COLORS,
    },
    snippets::utils::get_snippet_names,
};
use std::{fs, process};

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

    let mut snippet_path = snippets_dir();
    snippet_path.push(&snippet);

    match action.as_deref() {
        Some("Open") => {
            let _ = process::Command::new("nvim")
                .args([&snippet_path])
                .spawn()
                .expect("Failed to open neovim.")
                .wait();

            menu::clear_screen();
            process::exit(0);
        }
        Some("Copy") => {
            let mut clip = arboard::Clipboard::new().unwrap();
            let content = fs::read_to_string(&snippet_path).expect("Failed to open snippet.");
            clip.set_text(content).unwrap();

            process::exit(0);
        }
        Some("Delete") => {
            if let Some((choice, _)) = menu::run(&mut Cursor::new(
                "Delete snippet",
                Some(vec![&format!("Name: {}", &snippet), "Are you sure?", ""]),
                vec!["Yes", "No"],
            )) && choice == "No"
            {
                return;
            }

            fs::remove_file(snippet_path).unwrap();

            println!(
                "{white}{} {yellow}deleted.\n{reset}",
                snippet,
                white = COLORS.White,
                yellow = COLORS.Yellow,
                reset = COLORS.Reset
            );
        }
        _ => {}
    }
}
