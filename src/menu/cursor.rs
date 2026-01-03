use crate::terminal;

use terminal::console;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::{
    COORD, GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleCursorPosition,
};

pub struct Cursor<'a> {
    pub header: &'a str,
    pub subheaders: Vec<&'a str>,

    pub items: Vec<&'a str>,
    pub current: usize,
    offset: usize,
    pub visible_items: usize,
    pub total_height: usize,
    console_width: usize,

    pub stdout_handle: HANDLE,
}

impl<'a> Cursor<'a> {
    pub fn new(header: &'a str, subheaders: Option<Vec<&'a str>>, items: Vec<&'a str>) -> Self {
        let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

        let subheaders = subheaders.unwrap_or(vec![]);
        let visible_items = items.len().min(20);
        let total_height = 1 + subheaders.len() + visible_items;

        let console_width = console::get_console_width();

        Self {
            header,
            subheaders,
            items,
            current: 0,
            offset: 0,
            visible_items,
            total_height,
            console_width,
            stdout_handle: stdout,
        }
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

    pub fn increment(&mut self) {
        let offset = &self.offset;

        let pos_from_bottom = self.visible_items - (self.current - offset + 1);

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
