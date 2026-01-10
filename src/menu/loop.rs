use crate::menu;
use crate::terminal;

use menu::cursor;
use menu::input;

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};

pub fn run<'a>(
    header: &'a str,
    subheaders: Option<Vec<&'a str>>,
    items: Vec<&'a str>,
) -> Option<&'a str> {
    let stdin: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

    let mut cursor = cursor::Cursor::new(header, subheaders, items);
    terminal::set_cursor_visibility(false);

    menu::write_headers(cursor.header, Some(&cursor.subheaders));

    let mut start_y = terminal::get_cursor_position().Y;

    loop {
        cursor.set_cursor_pos(0, start_y);

        cursor.render_menu();

        let key = input::read_key_blocking(stdin);

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

        start_y = terminal::get_cursor_position().Y - cursor.visible_items as i16;
    }
}
