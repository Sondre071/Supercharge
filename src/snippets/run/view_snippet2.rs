use crate::{
    shared::{
        menu::{Cursor, NONE},
        statics,
        terminal::{self, ACTIONS, COLORS, read_key_blocking},
    },
    snippets::utils,
};
use crossterm::event::KeyCode;
use std::{fs, io::{Write, stdout}, iter, path::PathBuf, process};

pub fn view_snippet2() {
    terminal::set_cursor_visibility(false);

    let snippets_dir = statics::snippets_dir();
    let snippet_names = utils::get_snippet_names();

    let mut cursor = Cursor::new("Snippets", NONE, snippet_names);

    let mut selected_snippet: String;
    let mut start_y = terminal::get_cursor_pos().Y;

    loop {
        terminal::set_cursor_pos(0, start_y as usize);

        selected_snippet = cursor.items[cursor.current].value.clone();

        let mut lines = cursor.assemble_menu();
        
        lines.append(&mut assemble_view(&snippets_dir, &selected_snippet));
        
        #[allow(clippy::print_with_newline)]
        for line in &lines {
            print!("{}\n", line);
        }
        
        stdout().flush().unwrap();

        let key = read_key_blocking();

        match key {
            KeyCode::Char('q') | KeyCode::Char('h') => {
                //draw::clear_menu(rendered_items);
                terminal::set_cursor_visibility(true);

                process::exit(0);
            }

            KeyCode::Char('j') => {
                let offset = cursor.offset;

                let pos_from_bottom = cursor.visible_items - (cursor.current - offset + 1);

                if cursor.current < cursor.items.len() - 1 {
                    cursor.current += 1;
                }

                if pos_from_bottom == 1 && cursor.current < cursor.items.len() - 1 {
                    cursor.offset += 1
                }
            }
            KeyCode::Char('k') => {
                let pos_from_top = cursor.current - cursor.offset;

                if cursor.current > 0 {
                    cursor.current -= 1;
                }

                if cursor.offset > 0 && pos_from_top <= 1 {
                    cursor.offset -= 1
                }
            }

            KeyCode::Char('l') => {
                //draw::clear_menu(rendered_items);
                terminal::set_cursor_visibility(true);

                process::exit(0);
            }

            _ => {}
        }

        // Recalculcates in case terminal window scrolls during initial render.
        start_y = terminal::get_cursor_pos().Y - lines.len() as i16;
    }
}

fn assemble_view(file_directory: &PathBuf, file_name: &str) -> Vec<String> {
    let mut lines = Vec::<String>::new();

    let mut file_path = file_directory.to_owned();
    file_path.push(file_name);

    let border_color = COLORS.Gray;
    let text_color = COLORS.Cyan;

    lines.push(format!(
        "{clear_line}{border_color}│{reset}",
        clear_line = ACTIONS.ClearLine,
        border_color = border_color,
        reset = COLORS.Reset
    ));

    let header_text = {
        let width: usize = 30;

        // Truncate
        let mut header = file_name.to_owned();

        if header.chars().count() > width {
            let truncated: String = header.chars().take(width - 2).collect();
            header = format!("{truncated}..");
        }

        // Format
        let pad_left_len = (width.saturating_sub(header.chars().count()) - 2) / 2;
        let pad_right_len = width - pad_left_len - header.chars().count();

        let pad_left: String = iter::repeat_n("─", pad_left_len).collect();
        let pad_right: String = iter::repeat_n("─", pad_right_len).collect();

        format!(
            "├{color}{} {} {}{reset}",
            pad_left,
            header,
            pad_right,
            color = border_color,
            reset = COLORS.Reset
        )
    };

    lines.push(header_text);

    fs::read_to_string(file_path)
        .expect("Failed to read snippet content.")
        .lines()
        .take(10)
        .for_each(|l| {
            lines.push(format!(
                "{clear_line}{border_color}│ {text_color}{}{reset}",
                l,
                clear_line = ACTIONS.ClearLine,
                text_color = text_color,
                border_color = border_color,
                reset = COLORS.Reset
            ));
        });

    lines
}
