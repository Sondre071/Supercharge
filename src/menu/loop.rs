use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};

use super::cursor;
use super::input;

pub fn run<'a>(
    header: &'a str,
    subheaders: Option<Vec<&'a str>>,
    items: Vec<&'a str>,
) -> Option<&'a str> {
    let stdin: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

    let mut cursor = cursor::Cursor::new(header, subheaders, items);

    cursor.write_headers();

    let mut start_y = cursor.get_cursor_pos().dwCursorPosition.Y;

    loop {
        cursor.set_cursor_pos(0, start_y);
        cursor.render_menu();

        let key = input::read_key_blocking(stdin);

        if let Some(ch) = key.ch {
            match ch {
                'q' | 'h' => {
                    cursor.clear_menu();
                    return None;
                }

                'j' => {
                    cursor.increment();
                }
                'k' => {
                    cursor.decrement();
                }

                'l' => {
                    cursor.clear_menu();
                    return Some(cursor.items[cursor.current]);
                }

                _ => {
                    continue;
                }
            }
        }

        start_y = cursor.get_cursor_pos().dwCursorPosition.Y - cursor.height as i16;
    }
}
