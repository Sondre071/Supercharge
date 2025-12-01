use std::process::exit;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};

use super::cursor;
use super::input;

use super::Menu;
use super::MenuId;

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

pub fn run(menu: MenuId) -> Option<MenuId> {
    let stdin: HANDLE = unsafe { GetStdHandle(STD_INPUT_HANDLE) };

    let menu = lookup_menu(menu);
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
                    cursor.clear_menu();

                    return Some(back_menu(&menu));
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
                    cursor.clear_menu();
                    return Some((menu.options[cursor.current.get()].next)());
                }

                _ => {
                    continue;
                }
            }
        }

        start_y = cursor.get_cursor_pos().dwCursorPosition.Y - menu.options.len() as i16;
    }
}
