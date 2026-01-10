use std::mem::MaybeUninit;
use windows_sys::Win32::System::Console::{
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, GetConsoleScreenBufferInfo,
    GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleCursorInfo,
};

#[allow(dead_code)]
pub fn get_console_width() -> usize {
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let mut info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();

    let csbi = unsafe {
        if GetConsoleScreenBufferInfo(stdout, info.as_mut_ptr()) == 0 {
            panic!("Failed to get console screen buffer info.")
        }

        info.assume_init()
    };

    (csbi.srWindow.Right - csbi.srWindow.Left + 1) as usize
}

pub fn get_cursor_position() -> COORD {
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let mut info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();

    let csbi = unsafe {
        if GetConsoleScreenBufferInfo(stdout, info.as_mut_ptr()) == 0 {
            panic!("Failed to get console screen buffer info.")
        }

        info.assume_init()
    };

    csbi.dwCursorPosition
}

pub fn set_cursor_visibility(visible: bool) {
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let num = { if visible { 1 } else { 0 } };

    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: num,
    };
    unsafe {
        if SetConsoleCursorInfo(stdout, &cursor_info) == 0 {
            panic!("Could not set cursor info.");
        };
    }
}
