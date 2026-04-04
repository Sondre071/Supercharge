use crate::{
    shared::{
        menu::{self, Cursor, Focus, Item, NONE},
        statics,
        terminal::{self, ACTIONS, COLORS},
    },
    snippets::utils,
};
use std::{fs, iter};

fn assemble_snippets() -> Vec<Item> {
    let snippets_dir = statics::snippets_dir();
    let snippet_names = utils::get_snippet_names();

    snippet_names
        .into_iter()
        .map(|name| {
            let mut path = snippets_dir.clone();
            path.push(&name);

            let section_titles = fs::read_to_string(&path)
                .map(|content| {
                    utils::parse_sections(&content)
                        .into_iter()
                        .map(|s| s.title)
                        .collect()
                })
                .unwrap_or_default();

            Item::new_with_subitems(name, section_titles)
        })
        .collect()
}

pub fn view_snippets() {
    terminal::set_cursor_visibility(false);

    let mut cursor = Cursor::new_with_subitems(
        "Snippets",
        NONE,
        assemble_snippets(),
        Some(Box::new(display_func)),
    );

    let Some((file_name, Some(section_title))) = menu::run(&mut cursor) else {
        return;
    };

    let mut path = statics::snippets_dir();
    path.push(&file_name);

    let content = fs::read_to_string(&path).expect("Failed to read snippet file.");
    let sections = utils::parse_sections(&content);

    if let Some(section) = sections.into_iter().find(|s| s.title == section_title) {
        arboard::Clipboard::new()
            .unwrap()
            .set_text(section.content)
            .unwrap();
    }
}

fn display_func(cursor: &Cursor) -> Vec<String> {
    let mut lines = Vec::<String>::new();

    if matches!(cursor.focus, Focus::BaseMenu) {
        return lines;
    };

    let selected_snippet = cursor.items[cursor.current].value.clone();

    let file_path = {
        let mut pathbuf = statics::snippets_dir();
        pathbuf.push(&selected_snippet);

        pathbuf
    };

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
        let section_title = &cursor.items[cursor.current].items[cursor.submenu_current];
        let mut header = format!("## {}", section_title);

        if header.chars().count() > width - 6 {
            let truncated: String = header.chars().take(width - 6).collect();
            header = format!("{truncated}..");
        }

        // Format
        let pad_left_len = (width
            .saturating_sub(header.chars().count())
            .saturating_sub(2))
            / 2;
        let pad_right_len = width
            .saturating_sub(pad_left_len)
            .saturating_sub(header.chars().count());

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

    let content = fs::read_to_string(file_path).expect("Failed to read snippet content.");
    let sections = utils::parse_sections(&content);

    if let Some(section) = sections.get(cursor.submenu_current) {
        section.content.lines().take(10).for_each(|l| {
            lines.push(format!(
                "{clear_line}{border_color}│ {text_color}{}{reset}",
                l,
                clear_line = ACTIONS.ClearLine,
                text_color = text_color,
                reset = COLORS.Reset
            ));
        });
    }

    lines
}
