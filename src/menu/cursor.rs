use std::io::{self, Write};
use std::mem::MaybeUninit;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, GetConsoleScreenBufferInfo,
    GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleCursorInfo, SetConsoleCursorPosition,
};

pub struct Cursor<'a> {
    header: &'a str,
    subheaders: Vec<&'a str>,

    pub items: Vec<&'a str>,
    pub current: usize,
    offset: usize,
    pub height: usize,
    console_width: usize,

    stdout_handle: HANDLE,
}

impl<'a> Cursor<'a> {
    pub fn new(header: &'a str, subheaders: Option<Vec<&'a str>>, items: Vec<&'a str>) -> Self {
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

        let height = items.len().min(20);

        // Get console info.
        let csbi = unsafe { get_console_info(stdout).expect("Could not get console info.") };
        let console_width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as usize;

        Self {
            header,
            subheaders: subheaders.unwrap_or(vec![]),
            items: items,
            current: 0,
            offset: 0,
            height,
            console_width,
            stdout_handle: stdout,
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
        let rows_to_jump = 1 + self.subheaders.len() + self.items.len();

        print!("\x1b[{}A", rows_to_jump);
        print!("\x1b[0J");

        io::stdout().flush().unwrap();
    }

    pub fn render_menu(&self) {
        let content_width = self.console_width.saturating_sub(2);
        let height = self.items.len().min(20);
        let offset = self.offset;

        for i in offset..(height + offset) {
            if i == self.current {
                println!(
                    "\x1b[0;93m> {: <width$}\x1b[0m",
                    self.items[i],
                    width = content_width
                );
            } else {
                println!("  {: <width$}", self.items[i], width = content_width);
            }
        }
    }

    pub fn write_headers(&self) {
        let header_text = {
            let width: usize = 30;

            // Truncate
            let mut header_str = self.header.to_string();

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

        for subheader in self.subheaders.iter() {
            println!("\x1b[0;93m{}\x1b[0m", subheader)
        }
    }

    pub fn increment(&mut self) {
        let offset = &self.offset;

        let pos_from_bottom = self.height - (self.current - offset + 1);

        if self.current < self.items.len() - 1 {
            self.current += 1;
        }

        if pos_from_bottom == 1 && self.current < self.items.len() - 1 {
            self.offset += 1
        }
    }

    pub fn decrement(&mut self) {
        let pos_from_top = self.current - self.offset;

        if self.current > 0 {
            self.current -= 1;
        }

        if self.offset > 0 && pos_from_top <= 1 {
            self.offset -= 1
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
