use std::process::exit;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

use crate::{cursor, init, input};
//use std::io::{self, Write};

pub enum MenuId {
    Home,
    Settings,
    OpenRouter,
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
    subheaders: &[],
    options: &[Item {
        name: "Back",
        next: to_home,
    }],
};

struct Opt<'a> {
    index: i32,
    text: &'a str,
}

fn lookup_menu(id: MenuId) -> &'static Menu {
    match id {
        MenuId::Home => &HOME,
        MenuId::Settings => &SETTINGS,
        MenuId::OpenRouter => &OPENROUTER,
    }
}

pub fn run(start: MenuId) {
    let mut currentId = start;

    loop {
        let menu = lookup_menu(currentId);

        let stdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

        init::init_console(stdout);

        println!("\x1b[0;93m========== {} ==========\x1b[0m\n", menu.header);

        if (!menu.subheaders.is_empty()) {
            menu.subheaders
                .into_iter()
                .for_each(|subheader| println!("\x1b[0;93m{}\x1b[0m", subheader));
        }

        let cursor = cursor::Cursor::new(stdin, stdout, menu.options.len(), 5);

        let mut current_index = 0;
        let mut startRow = 0;

        render_menu(&menu.options, current_index, 90);

        loop {
            let key = input::read_key_blocking(stdin);

            cursor.clear_options();

            print!("\x1b[{}A\r", menu.options.len());

            if let Some(ch) = key.ch {
                match ch {
                    'q' | 'h' => exit(-1),

                    'j' => {
                        if current_index != menu.options.len() - 1 {
                            current_index += 1
                        }
                    }
                    'k' => {
                        if current_index != 0 {
                            current_index -= 1
                        }
                    }

                    'l' => {
                        currentId = (menu.options[current_index].next)();
                        clear_menu(5, menu.options.len());

                        break;
                    }

                    _ => continue,
                }
            }

            render_menu(&menu.options, current_index, 90);
        }
    }
}

fn render_menu(options: &&'static [Item], current: usize, width: usize) {
    let content_width = width.saturating_sub(2);

    for i in 0..options.len() {
        if i == current {
            println!(
                "\x1b[0;93m> {: <num$}\x1b[0m",
                options[i].name,
                num = content_width
            );
        } else {
            println!("  {: <num$}", options[i].name, num = content_width);
        }
    }
}

fn clear_menu(menu_len: usize, options_len: usize) {
    println!("\x1b[{}A", menu_len - options_len + 1);
    println!("\x1b[0J");
}
