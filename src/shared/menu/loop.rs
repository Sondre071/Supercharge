use crate::shared::{menu, terminal};

use menu::cursor;

pub fn run<'a>(
    header: &'a str,
    subheaders: Option<Vec<&'a str>>,
    items: Vec<&'a str>,
) -> Option<&'a str> {
    let mut cursor = cursor::Cursor::new(header, subheaders, items);
    terminal::set_cursor_visibility(false);

    menu::write_headers(cursor.header, Some(&cursor.subheaders));

    let mut start_y = terminal::get_cursor_pos().Y;

    loop {
        terminal::set_cursor_pos(0, start_y as usize);

        cursor.render_menu();

        let key = terminal::read_key_blocking();

        if let Some(ch) = key.ch {
            match ch {
                'q' | 'h' => {
                    menu::clear_menu(cursor.total_height);
                    terminal::set_cursor_visibility(true);
                    return None;
                }

                'j' => {
                    cursor.increment();
                }
                'k' => {
                    cursor.decrement();
                }

                'l' => {
                    menu::clear_menu(cursor.total_height);
                    terminal::set_cursor_visibility(true);
                    return Some(cursor.items[cursor.current]);
                }

                _ => {
                    continue;
                }
            }
        }

        start_y = terminal::get_cursor_pos().Y - cursor.visible_items as i16;
    }
}
