use crate::{
    shared::{
        menu,
        statics::snippets_dir,
        terminal::{self, read_key_blocking, ACTIONS, COLORS},
    },
    snippets::utils::parse_sections,
};
use crossterm::event::KeyCode;
use std::{fs, io::Write, process};

const PREVIEW_MAX_LINES: usize = 10;

pub fn view_snippet(file_name: &str) {
    let mut path = snippets_dir();
    path.push(format!("{}.md", file_name));

    let content = fs::read_to_string(&path).expect("Failed to read snippet file.");
    let sections = parse_sections(&content);

    if sections.is_empty() {
        println!(
            "{yellow}No sections found in {file_name}.{reset}",
            yellow = COLORS.Yellow,
            reset = COLORS.Reset,
        );
        return;
    }

    terminal::set_cursor_visibility(false);

    menu::write_headers(file_name, Vec::<String>::new());

    let start_y = terminal::get_cursor_pos().Y;
    let mut current: usize = 0;

    loop {
        terminal::set_cursor_pos(0, start_y as usize);
        print!("{}", ACTIONS.ClearToEnd);
        std::io::stdout().flush().unwrap();

        for (i, section) in sections.iter().enumerate() {
            if i == current {
                println!(
                    "{yellow}> {title}{reset}",
                    yellow = COLORS.Yellow,
                    title = section.title,
                    reset = COLORS.Reset,
                );
            } else {
                println!(
                    "{gray}  {title}{reset}",
                    gray = COLORS.Gray,
                    title = section.title,
                    reset = COLORS.Reset,
                );
            }
        }

        println!();

        let preview: Vec<&str> = sections[current]
            .content
            .lines()
            .take(PREVIEW_MAX_LINES)
            .collect();

        let truncated = sections[current].content.lines().count() > PREVIEW_MAX_LINES;

        for line in &preview {
            println!(
                "{white}{line}{reset}",
                white = COLORS.White,
                reset = COLORS.Reset,
            );
        }

        if truncated {
            println!(
                "{dark}...{reset}",
                dark = COLORS.DarkGray,
                reset = COLORS.Reset,
            );
        }

        std::io::stdout().flush().unwrap();

        match read_key_blocking() {
            KeyCode::Char('j') => {
                if current < sections.len() - 1 {
                    current += 1;
                }
            }
            KeyCode::Char('k') => {
                current = current.saturating_sub(1);
            }
            KeyCode::Char('l') | KeyCode::Enter => {
                let mut clip = arboard::Clipboard::new().unwrap();
                clip.set_text(sections[current].content.clone()).unwrap();

                terminal::set_cursor_pos(0, start_y as usize);
                print!("{}", ACTIONS.ClearToEnd);
                std::io::stdout().flush().unwrap();
                terminal::set_cursor_visibility(true);

                process::exit(0);
            }
            KeyCode::Char('q') | KeyCode::Char('h') => {
                terminal::set_cursor_pos(0, start_y as usize);
                print!("{}", ACTIONS.ClearToEnd);
                std::io::stdout().flush().unwrap();
                terminal::set_cursor_visibility(true);
                return;
            }
            _ => {}
        }
    }
}
