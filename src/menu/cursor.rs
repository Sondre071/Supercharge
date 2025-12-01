use std::cell::Cell;
use std::io::{self, Write};
use std::mem::MaybeUninit;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, GetConsoleScreenBufferInfo,
    GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleCursorInfo, SetConsoleCursorPosition,
};

use crate::menu;

pub struct Cursor<'a> {
    pub current: Cell<usize>,

    menu: &'a menu::Menu,
    width: usize,
    stdout_handle: HANDLE,
}

impl<'a> Cursor<'a> {
    pub fn new(menu: &'a menu::Menu) -> Self {
        let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

        // Init console.
        let cursor_info = CONSOLE_CURSOR_INFO {
            dwSize: 100,
            bVisible: 0,
        };
        unsafe {
            if SetConsoleCursorInfo(stdout, &cursor_info) == 0 {
                panic!("Could not get cursor info.");
            };
        }

        // Get screen info.
        let csbi = unsafe { get_console_info(stdout).expect("Could not get console info.") };

        let width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as usize;

        Self {
            stdout_handle: stdout,
            menu,
            width: width,
            current: Cell::new(0),
        }
    }

    pub fn get_cursor_pos(&self) -> CONSOLE_SCREEN_BUFFER_INFO {
        unsafe { get_console_info(self.stdout_handle).expect("Could not get console info.") }
    }

    pub fn set_cursor_pos(&self, x: i16, y: i16) {
        let pos = COORD {
            X: x as i16,
            Y: y as i16,
        };

        unsafe {
            if SetConsoleCursorPosition(self.stdout_handle, pos) == 0 {
                panic!("Could not set cursor position.");
            }
        };
    }

    pub fn clear_menu(&self) {
        let rows_to_jump = 1 + self.menu.subheaders.len() + self.menu.options.len();

        print!("\x1b[{}A", rows_to_jump);
        print!("\x1b[0J");

        io::stdout().flush().unwrap();
    }

    pub fn render_menu(&self) {
        let content_width = self.width.saturating_sub(2);

        for i in 0..self.menu.options.len() {
            if i == self.current.get() {
                println!(
                    "\x1b[0;93m> {: <width$}\x1b[0m",
                    self.menu.options[i].name,
                    width = content_width
                );
            } else {
                println!(
                    "  {: <width$}",
                    self.menu.options[i].name,
                    width = content_width
                );
            }
        }
    }

    pub fn write_headers(&self) {
        let header_text = {
            let width: usize = 30;

            // Truncate
            let mut header_str = self.menu.header.to_string();

            if header_str.chars().count() > width {
                let truncated: String = header_str.chars().take(width - 2).collect();
                header_str = format!("{truncated}..");
            }

            let pad_left_len = (width.saturating_sub(header_str.chars().count()) - 2) / 2;
            let pad_right_len = width - pad_left_len - header_str.chars().count();

            let pad_left: String = std::iter::repeat("=").take(pad_left_len).collect();
            let pad_right: String = std::iter::repeat("=").take(pad_right_len).collect();
            format!("\x1b[0;93m{} {} {}\x1b[0m", pad_left, header_str, pad_right)
        };

        println!("{header_text}");

        for subheader in self.menu.subheaders.iter() {
            println!("\x1b[0;93m{}\x1b[0m", subheader)
        }
    }
}

unsafe fn get_console_info(handle: HANDLE) -> Option<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();

    unsafe {
        if GetConsoleScreenBufferInfo(handle, info.as_mut_ptr()) == 0 {
            None
        } else {
            Some(info.assume_init())
        }
    }
}
