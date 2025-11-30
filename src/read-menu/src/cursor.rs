use std::mem::MaybeUninit;
use windows_sys::Win32::System::Console::{
    CONSOLE_SCREEN_BUFFER_INFO, COORD, GetConsoleScreenBufferInfo, SetConsoleCursorPosition,
};

use windows_sys::Win32::Foundation::HANDLE;

pub struct Cursor {
    stdin_handle: HANDLE,
    stdout_handle: HANDLE,
    options_size: i16,
    menu_size: i16,
}

impl Cursor {
    pub fn new(stdin_handle: HANDLE, stdout_handle: HANDLE, options_size: usize, menu_size: usize) -> Self {
        Self {
            stdin_handle,
            stdout_handle,
            options_size: options_size as i16,
            menu_size: menu_size as i16,
        }
    }

    pub fn clear_options(&self) {
        unsafe {
            let mut screen_buffer_info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();

            if GetConsoleScreenBufferInfo(self.stdin_handle, screen_buffer_info.as_mut_ptr()) == 1 {
                let csbi = screen_buffer_info.assume_init();

                println!("{:?}", self.options_size);

                let pos = COORD {
                    X: 0,
                    Y: csbi.dwCursorPosition.Y - self.options_size,
                };

                SetConsoleCursorPosition(self.stdout_handle, pos);

                println!("\x1b[0J");
            }
        }
    }
}
