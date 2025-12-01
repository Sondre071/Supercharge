use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};

use std::process::exit;

use crate::{cursor, input};
//use std::io::{self, Write};

pub enum MenuId {
    Home,
    Settings,
    OpenRouter,

    Quit,
}

pub fn to_home() -> MenuId {
    MenuId::Home
}

pub fn to_settings() -> MenuId {
    MenuId::Settings
}

pub fn to_openrouter() -> MenuId {
    MenuId::OpenRouter
}

pub fn to_quit() -> MenuId {
    MenuId::Quit
}

pub struct Item {
    pub name: &'static str,
    pub next: fn() -> MenuId,
}

pub struct Menu {
    pub header: &'static str,
    pub subheaders: &'static [&'static str],
    pub options: &'static [Item],
}

static HOME: Menu = Menu {
    header: "Supercharge",
    subheaders: &[],
    options: &[
        Item {
            name: "OpenRouter",
            next: to_openrouter,
        },
        Item {
            name: "Settings",
            next: to_settings,
        },
        Item {
            name: "Quit",
            next: to_quit,
        },
    ],
};

static SETTINGS: Menu = Menu {
    header: "Settings",
    subheaders: &[],
    options: &[Item {
        name: "Back",
        next: to_home,
    }],
};

static OPENROUTER: Menu = Menu {
    header: "OpenRouter",
    subheaders: &["Her kan du chatte!", ""],
    options: &[Item {
        name: "Back",
        next: to_home,
    }],
};

fn back_menu(menu: &Menu) -> MenuId {
    menu.options
        .last()
        .map(|item| (item.next)())
        .unwrap_or(MenuId::Home)
}

fn lookup_menu(id: MenuId) -> &'static Menu {
    match id {
        MenuId::Home => &HOME,
        MenuId::Settings => &SETTINGS,
        MenuId::OpenRouter => &OPENROUTER,
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
