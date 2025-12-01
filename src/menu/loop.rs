use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};
use std::process::exit;

use super::cursor;
use super::input;

use super::MenuId;
use super::Menu;

//use std::io::{self, Write};



fn back_menu(menu: &Menu) -> MenuId {
    menu.options
        .last()
        .map(|item| (item.next)())
        .unwrap_or(MenuId::Home)
}

fn lookup_menu(id: MenuId) -> &'static Menu {
    match id {
        MenuId::Home => &super::HOME,
        MenuId::Settings => &super::SETTINGS,
        MenuId::OpenRouter => &super::OPENROUTER,
        MenuId::Quit => exit(0),
    }
}

pub fn run(start: MenuId) {
    let mut current_id: MenuId = start;

    let stdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

    loop {
        let menu = lookup_menu(current_id);
        let cursor = cursor::Cursor::new(&menu);

        cursor.write_headers();

        let mut start_y = cursor.get_cursor_pos().dwCursorPosition.Y;

        loop {
            cursor.set_cursor_pos(0, start_y);
            cursor.render_menu();

            let key = input::read_key_blocking(stdin);

            if let Some(ch) = key.ch {
                match ch {
                    'q' | 'h' => {
                        current_id = back_menu(&menu);
                        cursor.clear_menu();

                        break;
                    }

                    'j' => {
                        if cursor.current.get() < menu.options.len() - 1 {
                            cursor.current.set(cursor.current.get() + 1);
                        }
                    }
                    'k' => {
                        if cursor.current.get() > 0 {
                            cursor.current.set(cursor.current.get() - 1);
                        }
                    }

                    'l' => {
                        current_id = (menu.options[cursor.current.get()].next)();
                        cursor.clear_menu();

                        break;
                    }

                    _ => {
                        continue;
                    }
                }
            }

            start_y = cursor.get_cursor_pos().dwCursorPosition.Y - menu.options.len() as i16;
        }
    }
}
