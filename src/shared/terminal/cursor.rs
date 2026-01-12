use std::mem::MaybeUninit;
use windows_sys::Win32::System::Console::{
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, COORD, GetConsoleScreenBufferInfo,
    GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleCursorInfo, SetConsoleCursorPosition,
};

pub fn get_cursor_pos() -> COORD {
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

pub fn set_cursor_pos(x: usize, y: usize) {
    let pos: COORD = COORD {
        X: x as i16,
        Y: y as i16,
    };

    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    unsafe {
        if SetConsoleCursorPosition(stdout, pos) == 0 {
            panic!("Could not set cursor position.");
        }
    };
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

pub fn move_cursor_pos(x: Option<i16>, y: Option<i16>) {
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let mut info = MaybeUninit::<CONSOLE_SCREEN_BUFFER_INFO>::uninit();

    let csbi = unsafe {
        if GetConsoleScreenBufferInfo(stdout, info.as_mut_ptr()) == 0 {
            panic!("Failed to get console screen buffer info.");
        }

        info.assume_init()
    };

    let mut pos = csbi.dwCursorPosition.to_owned();

    if let Some(val) = x {
        pos.X += val;
    }

    if let Some(val) = y {
        pos.Y += val;
    }

    unsafe {
        if SetConsoleCursorPosition(stdout, pos) == 0 {
            panic!("Could not set cursor position.");
        }
    };
}
